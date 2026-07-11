package kernel

import (
	"github.com/robertpelloni/bcs/ports/go/core/global"
	"github.com/robertpelloni/bcs/ports/go/core/kernel"
	"sync"
)

// Focus Scope Constants
const (
	ScopeWidget      = 0
	ScopeWindow      = 1
	ScopeApplication = 2
	ScopeWorkspace   = 3
	ScopeGlobal      = 4
)

// Modality Policy Constants
const (
	PolicyShared               = 0
	PolicyOwnerExclusive       = 1
	PolicyWindowExclusive      = 2
	PolicyApplicationExclusive = 3
	PolicySystemExclusive      = 4
)

// BcsInputOwner maps the conceptual multi-user network ID to an input source
type BcsInputOwner struct {
	ID   string
	Name string
}

func NewBcsInputOwner(id, name string) *BcsInputOwner {
	return &BcsInputOwner{ID: id, Name: name}
}

// BcsFocusToken represents an explicit grant of focus for a given owner and surface.
type BcsFocusToken struct {
	TokenID        string
	OwnerID        string
	SurfaceID      string
	Scope          int
	ModalityPolicy int
	Priority       int
	Active         bool
}

func NewBcsFocusToken(tokenID, ownerID, surfaceID string) *BcsFocusToken {
	return &BcsFocusToken{
		TokenID:        tokenID,
		OwnerID:        ownerID,
		SurfaceID:      surfaceID,
		Scope:          ScopeWidget,
		ModalityPolicy: PolicyShared,
		Priority:       0,
		Active:         true,
	}
}

// Route request and result structures
const (
	RouteDecisionReject   = 0
	RouteDecisionShare    = 1
	RouteDecisionTransfer = 2
	RouteDecisionQueue    = 3
)

type BcsInputRouteRequest struct {
	OwnerID   string
	SurfaceID string
	DeviceID  string
	Kind      int
}

type BcsInputRouteResult struct {
	Decision          int
	ResolvedOwnerID   string
	BlockingOwnerID   string
	BlockingSurfaceID string
}

func (r *BcsInputRouteResult) Accepted() bool {
	return r.Decision != RouteDecisionReject
}

// BcsInputArbitrator handles multi-user event streams and focus delegation natively
type BcsInputArbitrator struct {
	mu           sync.RWMutex
	activeOwners map[string]*BcsInputOwner
	focusTokens  []*BcsFocusToken
	FocusChanged global.BcsSignal
}

func NewBcsInputArbitrator() *BcsInputArbitrator {
	return &BcsInputArbitrator{
		activeOwners: make(map[string]*BcsInputOwner),
		focusTokens:  make([]*BcsFocusToken, 0),
	}
}

// RegisterOwner adds a new network or local user to the arbitrator's pool
func (a *BcsInputArbitrator) RegisterOwner(owner *BcsInputOwner) {
	a.mu.Lock()
	defer a.mu.Unlock()
	a.activeOwners[owner.ID] = owner
}

func (a *BcsInputArbitrator) AddFocusToken(token *BcsFocusToken) {
	a.mu.Lock()
	defer a.mu.Unlock()
	a.focusTokens = append(a.focusTokens, token)
}

func (a *BcsInputArbitrator) WouldBlock(ownerID, surfaceID string) bool {
	a.mu.RLock()
	defer a.mu.RUnlock()

	// Simplified policy matching C++ btkinputarbitrator.cpp logic
	highest := a.highestPriorityActiveToken()
	if highest == nil {
		return false
	}

	if highest.ModalityPolicy == PolicyShared {
		return false
	}

	if highest.OwnerID == ownerID {
		return false
	}

	return true
}

func (a *BcsInputArbitrator) Route(request *BcsInputRouteRequest) *BcsInputRouteResult {
	a.mu.RLock()
	defer a.mu.RUnlock()

	result := &BcsInputRouteResult{Decision: RouteDecisionShare}

	highest := a.highestPriorityActiveToken()
	if highest == nil {
		return result
	}

	if highest.ModalityPolicy != PolicyShared && highest.OwnerID != request.OwnerID {
		result.Decision = RouteDecisionReject
		result.BlockingOwnerID = highest.OwnerID
		result.BlockingSurfaceID = highest.SurfaceID
	}

	return result
}

func (a *BcsInputArbitrator) highestPriorityActiveToken() *BcsFocusToken {
	var highest *BcsFocusToken
	for _, token := range a.focusTokens {
		if token.Active {
			if highest == nil || token.Priority > highest.Priority {
				highest = token
			}
		}
	}
	return highest
}

// RequestFocus attempts to route the target focus token to the specified owner
func (a *BcsInputArbitrator) RequestFocus(ownerID string, targetWidget *kernel.BcsObject) bool {
	a.mu.RLock()
	owner, exists := a.activeOwners[ownerID]
	a.mu.RUnlock()

	if !exists || targetWidget == nil {
		return false
	}

	// Emit signal broadcasting focus shift logic
	a.FocusChanged.Emit(owner)
	return true
}
