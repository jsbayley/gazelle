package cmd

import (
	"encoding/json"
	"fmt"
	"math"
	"os"

	"github.com/jsbayley/gazelle/pkg/core"
	"github.com/spf13/cobra"
)

var infoCmd = &cobra.Command{
	Use:   "info [model-file]",
	Short: "Show model information and statistics",
	Long:  `Display detailed information about a structural model including nodes, elements, materials, loads, and constraints.`,
	Args:  cobra.ExactArgs(1),
	RunE:  runInfo,
}

func init() {
	rootCmd.AddCommand(infoCmd)
}

func runInfo(cmd *cobra.Command, args []string) error {
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

	// Print detailed information
	fmt.Printf("🦌 Gazelle Model Information\n")
	fmt.Printf("===========================\n")
	fmt.Printf("File: %s\n", modelFile)
	fmt.Printf("%s\n", model.Summary())

	// Show coordinate bounds
	if len(model.Nodes) > 0 {
		minX, maxX := math.Inf(1), math.Inf(-1)
		minY, maxY := math.Inf(1), math.Inf(-1)
		minZ, maxZ := math.Inf(1), math.Inf(-1)

		for _, node := range model.Nodes {
			if node.X < minX {
				minX = node.X
			}
			if node.X > maxX {
				maxX = node.X
			}
			if node.Y < minY {
				minY = node.Y
			}
			if node.Y > maxY {
				maxY = node.Y
			}
			if node.Z < minZ {
				minZ = node.Z
			}
			if node.Z > maxZ {
				maxZ = node.Z
			}
		}

		fmt.Printf("Geometric bounds:\n")
		fmt.Printf("  X: %.3f to %.3f (span: %.3f)\n", minX, maxX, maxX-minX)
		fmt.Printf("  Y: %.3f to %.3f (span: %.3f)\n", minY, maxY, maxY-minY)
		fmt.Printf("  Z: %.3f to %.3f (span: %.3f)\n", minZ, maxZ, maxZ-minZ)
	}

	// Show sample nodes
	fmt.Printf("\nSample nodes:\n")
	count := 0
	for _, node := range model.Nodes {
		if count < 3 {
			fmt.Printf("  %s: (%.3f, %.3f, %.3f)\n", node.ID, node.X, node.Y, node.Z)
			count++
		}
	}
	if len(model.Nodes) > 3 {
		fmt.Printf("  ... and %d more nodes\n", len(model.Nodes)-3)
	}

	// Show materials
	if len(model.Materials) > 0 {
		fmt.Printf("\nMaterials:\n")
		for _, material := range model.Materials {
			fmt.Printf("  %s: %s (E=%.1e Pa)\n",
				material.ID, material.Name, material.ElasticModulus)
		}
	}

	// Show loads summary
	if len(model.Loads) > 0 {
		fmt.Printf("\nLoads summary:\n")
		totalForce := 0.0
		for _, load := range model.Loads {
			totalForce += math.Abs(load.Magnitude)
		}
		fmt.Printf("  Total applied force: %.1e N\n", totalForce)
	}

	return nil
}
