package kernel

import (
	"bcs/core/global"
	"sync"
)

// BcsInputOwner maps the conceptual multi-user network ID to an input source
type BcsInputOwner struct {
	ID   string
	Name string
}

func NewBcsInputOwner(id, name string) *BcsInputOwner {
	return &BcsInputOwner{ID: id, Name: name}
}

// BcsInputArbitrator handles multi-user event streams and focus delegation natively
type BcsInputArbitrator struct {
	mu           sync.RWMutex
	activeOwners map[string]*BcsInputOwner
	FocusChanged global.BcsSignal
}

func NewBcsInputArbitrator() *BcsInputArbitrator {
	return &BcsInputArbitrator{
		activeOwners: make(map[string]*BcsInputOwner),
	}
}

// RegisterOwner adds a new network or local user to the arbitrator's pool
func (a *BcsInputArbitrator) RegisterOwner(owner *BcsInputOwner) {
	a.mu.Lock()
	defer a.mu.Unlock()
	a.activeOwners[owner.ID] = owner
}

// RequestFocus attempts to route the target focus token to the specified owner
func (a *BcsInputArbitrator) RequestFocus(ownerID string, targetWidget *BcsObject) bool {
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
