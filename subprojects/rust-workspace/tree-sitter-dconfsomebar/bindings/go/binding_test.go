package tree_sitter_dconfsomebar_test

import (
	"testing"

	tree_sitter "github.com/smacker/go-tree-sitter"
	"github.com/tree-sitter/tree-sitter-dconfsomebar"
)

func TestCanLoadGrammar(t *testing.T) {
	language := tree_sitter.NewLanguage(tree_sitter_dconfsomebar.Language())
	if language == nil {
		t.Errorf("Error loading Dconfsomebar grammar")
	}
}
