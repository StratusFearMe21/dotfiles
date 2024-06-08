package tree_sitter_dconfdwl_test

import (
	"testing"

	tree_sitter "github.com/smacker/go-tree-sitter"
	"github.com/tree-sitter/tree-sitter-dconfdwl"
)

func TestCanLoadGrammar(t *testing.T) {
	language := tree_sitter.NewLanguage(tree_sitter_dconfdwl.Language())
	if language == nil {
		t.Errorf("Error loading Dconfdwl grammar")
	}
}
