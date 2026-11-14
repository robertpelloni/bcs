using System.Collections.Concurrent;
using System.Collections.Generic;
using Bcs.Core.Global;
using Bcs.Core.Kernel;

namespace Bcs.Gui.Kernel {
    public enum FocusScope {
        Widget, Window, Application, Workspace, Global
    }

    public enum ModalityPolicy {
        Shared, OwnerExclusive, WindowExclusive, ApplicationExclusive, SystemExclusive
    }

    public class BcsInputOwner {
        public string Id { get; set; }
        public string Name { get; set; }

        public BcsInputOwner(string id, string name) {
            Id = id;
            Name = name;
        }
    }

    public class BcsFocusToken {
        public string TokenId { get; set; }
        public string OwnerId { get; set; }
        public string SurfaceId { get; set; }
        public FocusScope Scope { get; set; }
        public ModalityPolicy ModalityPolicy { get; set; }
        public int Priority { get; set; }
        public bool Active { get; set; }

        public BcsFocusToken(string tokenId, string ownerId, string surfaceId) {
            TokenId = tokenId;
            OwnerId = ownerId;
            SurfaceId = surfaceId;
            Scope = FocusScope.Widget;
            ModalityPolicy = ModalityPolicy.Shared;
            Priority = 0;
            Active = true;
        }
    }

    public enum RouteDecision { Reject, Share, Transfer, Queue }

    public class BcsInputRouteRequest {
        public string OwnerId { get; set; }
        public string SurfaceId { get; set; }
        public string DeviceId { get; set; }
        public int Kind { get; set; }
    }

    public class BcsInputRouteResult {
        public RouteDecision Decision { get; set; } = RouteDecision.Share;
        public string ResolvedOwnerId { get; set; }
        public string BlockingOwnerId { get; set; }
        public string BlockingSurfaceId { get; set; }

        public bool Accepted() {
            return Decision != RouteDecision.Reject;
        }
    }

    public class BcsInputArbitrator {
        private ConcurrentDictionary<string, BcsInputOwner> _activeOwners;
        private List<BcsFocusToken> _focusTokens;
        private readonly object _lock = new object();
        public BcsSignal<BcsInputOwner> FocusChanged { get; private set; }

        public BcsInputArbitrator() {
            _activeOwners = new ConcurrentDictionary<string, BcsInputOwner>();
            _focusTokens = new List<BcsFocusToken>();
            FocusChanged = new BcsSignal<BcsInputOwner>();
        }

        public void RegisterOwner(BcsInputOwner owner) {
            _activeOwners[owner.Id] = owner;
        }

        public void AddFocusToken(BcsFocusToken token) {
            lock (_lock) {
                _focusTokens.Add(token);
            }
        }

        private BcsFocusToken HighestPriorityActiveToken() {
            BcsFocusToken highest = null;
            lock (_lock) {
                foreach (var token in _focusTokens) {
                    if (token.Active) {
                        if (highest == null || token.Priority > highest.Priority) {
                            highest = token;
                        }
                    }
                }
            }
            return highest;
        }

        public bool WouldBlock(string ownerId, string surfaceId) {
            var highest = HighestPriorityActiveToken();
            if (highest != null) {
                if (highest.ModalityPolicy != ModalityPolicy.Shared && highest.OwnerId != ownerId) {
                    return true;
                }
            }
            return false;
        }

        public BcsInputRouteResult Route(BcsInputRouteRequest request) {
            var result = new BcsInputRouteResult();
            var highest = HighestPriorityActiveToken();

            if (highest != null) {
                if (highest.ModalityPolicy != ModalityPolicy.Shared && highest.OwnerId != request.OwnerId) {
                    result.Decision = RouteDecision.Reject;
                    result.BlockingOwnerId = highest.OwnerId;
                    result.BlockingSurfaceId = highest.SurfaceId;
                }
            }
            return result;
        }

        public bool RequestFocus(string ownerId, BcsObject targetWidget) {
            if (!_activeOwners.TryGetValue(ownerId, out var owner) || targetWidget == null) {
                return false;
            }

            FocusChanged.Emit(owner);
            return true;
        }
    }
}
