package core

import (
	"encoding/json"
	"testing"

	"github.com/stretchr/testify/require"
)

// Void's wire encoding is JSON null: the scalar represents the absence of a
// value, and its literal encoding is already null. Without an explicit
// marshaller, encoding/json's empty-struct default ({}) reaches the wire,
// which strict clients rightly refuse to decode as an absent value.
func TestVoidMarshalsAsNull(t *testing.T) {
	out, err := json.Marshal(Void{})
	require.NoError(t, err)
	require.Equal(t, "null", string(out))
}
