package main

import (
	"bytes"
	"testing"
)

func Test_self_describe_hash(t *testing.T) {
	var buf bytes.Buffer
	self_describe_hash(&buf)
	if buf.String() != "The SHA256 for this sentence begins with: one, eight, two, a, seven, c and nine.\n" +
		"182a7c930b0e5227ff8d24b5f4500ff2fa3ee1a57bd35e52d98c6e24c2749ae0\n" {
		t.Errorf("wrong")
	}
}
