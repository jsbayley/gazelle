package cmd

import (
	"encoding/json"
	"fmt"
	"os"

	"github.com/jsbayley/gazelle/pkg/core"
	"github.com/spf13/cobra"
)

var validateCmd = &cobra.Command{
	Use:   "validate [model-file]",
	Short: "Validate a structural model",
	Long:  `Check a structural model for errors, inconsistencies, and completeness.`,
	Args:  cobra.ExactArgs(1),
	RunE:  runValidate,
}

func init() {
	rootCmd.AddCommand(validateCmd)
}

func runValidate(cmd *cobra.Command, args []string) error {
	modelFile := args[0]

	// Load model
	data, err := os.ReadFile(modelFile)
	if err != nil {
		return fmt.Errorf("failed to read model file: %w", err)
	}

	var model core.Model
	if err := json.Unmarshal(data, &model); err != nil {
		return fmt.Errorf("failed to parse model: %w", err)
	}

	fmt.Printf("🦌 Validating model: %s\n", modelFile)
	fmt.Printf("=======================\n")

	// Perform validation
	if err := model.Validate(); err != nil {
		fmt.Printf("❌ Validation failed: %s\n", err.Error())
		return nil
	}

	fmt.Printf("✅ Model validation passed!\n")
	fmt.Printf("%s", model.Summary())

	// Additional checks
	fmt.Printf("\nAdditional checks:\n")

	// Check for unconnected nodes
	connectedNodes := make(map[string]bool)
	for _, element := range model.Elements {
		for _, nodeID := range element.Nodes {
			connectedNodes[nodeID] = true
		}
	}

	unconnectedCount := 0
	for nodeID := range model.Nodes {
		if !connectedNodes[nodeID] {
			unconnectedCount++
		}
	}

	if unconnectedCount > 0 {
		fmt.Printf("⚠️  Warning: %d unconnected nodes found\n", unconnectedCount)
	} else {
		fmt.Printf("✅ All nodes are connected to elements\n")
	}

	// Check for nodes without constraints or loads
	constrainedNodes := make(map[string]bool)
	loadedNodes := make(map[string]bool)

	for _, constraint := range model.Constraints {
		constrainedNodes[constraint.Node] = true
	}

	for _, load := range model.Loads {
		if load.Node != "" {
			loadedNodes[load.Node] = true
		}
	}

	freeNodes := 0
	for nodeID := range model.Nodes {
		if !constrainedNodes[nodeID] && !loadedNodes[nodeID] {
			freeNodes++
		}
	}

	if freeNodes > 0 {
		fmt.Printf("⚠️  Info: %d nodes have no constraints or loads\n", freeNodes)
	}

	// Check constraint sufficiency
	if len(model.Constraints) == 0 {
		fmt.Printf("❌ Error: No constraints defined - model will be unstable\n")
	} else {
		fmt.Printf("✅ %d constraints defined\n", len(model.Constraints))
	}

	// Check load cases
	if len(model.Loads) == 0 {
		fmt.Printf("⚠️  Warning: No loads defined\n")
	} else {
		fmt.Printf("✅ %d loads defined\n", len(model.Loads))
	}

	fmt.Printf("\nModel appears ready for analysis! 🚀\n")
	return nil
}
