package cmd

import (
	"encoding/json"
	"fmt"
	"os"
	"path/filepath"

	"github.com/jsbayley/gazelle/pkg/core"
	"github.com/sirupsen/logrus"
	"github.com/spf13/cobra"
)

var (
	example string
	span    float64
	height  float64
	loads   []float64
)

var createCmd = &cobra.Command{
	Use:   "create [output-file]",
	Short: "Create a new structural model",
	Long: `Create a new structural analysis model from templates or examples.

Examples:
  gz create model.json --example truss
  gz create beam.json --example cantilever --span 10.0 --loads 50.0
  gz create frame.json --example portal --height 4.0 --span 12.0`,
	Args: cobra.ExactArgs(1),
	RunE: runCreate,
}

func init() {
	createCmd.Flags().StringVarP(&example, "example", "e", "truss", "example type (truss, cantilever, portal)")
	createCmd.Flags().Float64VarP(&span, "span", "s", 5.0, "span length in meters")
	createCmd.Flags().Float64Var(&height, "height", 3.0, "height in meters")
	createCmd.Flags().Float64SliceVarP(&loads, "loads", "l", []float64{10.0}, "load values in kN")

	rootCmd.AddCommand(createCmd)
}

func runCreate(cmd *cobra.Command, args []string) error {
	outputFile := args[0]

	logrus.Infof("Creating %s model: %s", example, outputFile)

	var model *core.Model
	var err error

	switch example {
	case "truss":
		model, err = createTrussExample()
	case "cantilever":
		model, err = createCantileverExample()
	case "portal":
		model, err = createPortalFrameExample()
	default:
		return fmt.Errorf("unknown example type: %s", example)
	}

	if err != nil {
		return fmt.Errorf("failed to create model: %w", err)
	}

	if err := saveModel(model, outputFile); err != nil {
		return fmt.Errorf("failed to save model: %w", err)
	}

	fmt.Printf("✓ Model created successfully: %s\n", outputFile)
	fmt.Printf("  Type: %s\n", example)
	fmt.Printf("  Nodes: %d\n", len(model.Nodes))
	fmt.Printf("  Elements: %d\n", len(model.Elements))
	fmt.Printf("  Materials: %d\n", len(model.Materials))

	return nil
}

func createTrussExample() (*core.Model, error) {
	model := core.NewModel("Simple Truss", "3-node triangular truss example")

	// Add steel material
	steel := &core.Material{
		ID:             "steel",
		Name:           "Structural Steel",
		Type:           core.MaterialSteel,
		ElasticModulus: 200e9, // 200 GPa
		PoissonRatio:   0.3,
		Density:        7850,  // kg/m³
		YieldStrength:  355e6, // 355 MPa
	}
	model.Materials["steel"] = steel

	// Add nodes
	model.Nodes["n1"] = &core.Node{ID: "n1", X: 0.0, Y: 0.0, Z: 0.0}
	model.Nodes["n2"] = &core.Node{ID: "n2", X: span, Y: 0.0, Z: 0.0}
	model.Nodes["n3"] = &core.Node{ID: "n3", X: span / 2, Y: height, Z: 0.0}

	// Add truss elements
	model.Elements["e1"] = &core.Element{
		ID:         "e1",
		Type:       core.ElementTruss2D,
		Nodes:      []string{"n1", "n2"},
		Material:   "steel",
		Properties: map[string]float64{"area": 0.01}, // 10 cm²
	}
	model.Elements["e2"] = &core.Element{
		ID:         "e2",
		Type:       core.ElementTruss2D,
		Nodes:      []string{"n2", "n3"},
		Material:   "steel",
		Properties: map[string]float64{"area": 0.01},
	}
	model.Elements["e3"] = &core.Element{
		ID:         "e3",
		Type:       core.ElementTruss2D,
		Nodes:      []string{"n3", "n1"},
		Material:   "steel",
		Properties: map[string]float64{"area": 0.01},
	}

	// Add constraints (pin supports)
	model.Constraints["c1"] = &core.Constraint{
		ID:   "c1",
		Node: "n1",
		Type: core.ConstraintFixed,
		DOF:  []string{"Ux", "Uy"},
	}
	model.Constraints["c2"] = &core.Constraint{
		ID:   "c2",
		Node: "n2",
		Type: core.ConstraintPinned,
		DOF:  []string{"Uy"},
	}

	// Add load (downward force at apex)
	loadValue := 10.0
	if len(loads) > 0 {
		loadValue = loads[0]
	}

	model.Loads["l1"] = &core.Load{
		ID:        "l1",
		Type:      core.LoadForce,
		Node:      "n3",
		Direction: "Fy",
		Magnitude: -loadValue * 1000, // Convert kN to N
	}

	return model, nil
}

func createCantileverExample() (*core.Model, error) {
	model := core.NewModel("Cantilever Beam", "Simple cantilever beam example")

	// Add steel material
	steel := &core.Material{
		ID:             "steel",
		Name:           "Structural Steel",
		Type:           core.MaterialSteel,
		ElasticModulus: 200e9,
		PoissonRatio:   0.3,
		Density:        7850,
		YieldStrength:  355e6,
	}
	model.Materials["steel"] = steel

	// Add nodes
	model.Nodes["n1"] = &core.Node{ID: "n1", X: 0.0, Y: 0.0, Z: 0.0}
	model.Nodes["n2"] = &core.Node{ID: "n2", X: span, Y: 0.0, Z: 0.0}

	// Add beam element
	model.Elements["e1"] = &core.Element{
		ID:       "e1",
		Type:     core.ElementBeam2D,
		Nodes:    []string{"n1", "n2"},
		Material: "steel",
		Properties: map[string]float64{
			"area":    0.01,     // 10 cm²
			"inertia": 8.333e-5, // I = bh³/12 for 200x300 beam
		},
	}

	// Add fixed constraint at base
	model.Constraints["c1"] = &core.Constraint{
		ID:   "c1",
		Node: "n1",
		Type: core.ConstraintFixed,
		DOF:  []string{"Ux", "Uy", "Rz"},
	}

	// Add load at tip
	loadValue := 50.0
	if len(loads) > 0 {
		loadValue = loads[0]
	}

	model.Loads["l1"] = &core.Load{
		ID:        "l1",
		Type:      core.LoadForce,
		Node:      "n2",
		Direction: "Fy",
		Magnitude: -loadValue * 1000, // Convert kN to N
	}

	return model, nil
}

func createPortalFrameExample() (*core.Model, error) {
	model := core.NewModel("Portal Frame", "Simple portal frame example")

	// Add steel material
	steel := &core.Material{
		ID:             "steel",
		Name:           "Structural Steel",
		Type:           core.MaterialSteel,
		ElasticModulus: 200e9,
		PoissonRatio:   0.3,
		Density:        7850,
		YieldStrength:  355e6,
	}
	model.Materials["steel"] = steel

	// Add nodes for portal frame
	model.Nodes["n1"] = &core.Node{ID: "n1", X: 0.0, Y: 0.0, Z: 0.0}     // Left base
	model.Nodes["n2"] = &core.Node{ID: "n2", X: 0.0, Y: height, Z: 0.0}  // Left top
	model.Nodes["n3"] = &core.Node{ID: "n3", X: span, Y: height, Z: 0.0} // Right top
	model.Nodes["n4"] = &core.Node{ID: "n4", X: span, Y: 0.0, Z: 0.0}    // Right base

	// Add frame elements
	model.Elements["e1"] = &core.Element{ // Left column
		ID:       "e1",
		Type:     core.ElementFrame2D,
		Nodes:    []string{"n1", "n2"},
		Material: "steel",
		Properties: map[string]float64{
			"area":    0.02,     // 20 cm²
			"inertia": 1.667e-4, // Column moment of inertia
		},
	}
	model.Elements["e2"] = &core.Element{ // Beam
		ID:       "e2",
		Type:     core.ElementFrame2D,
		Nodes:    []string{"n2", "n3"},
		Material: "steel",
		Properties: map[string]float64{
			"area":    0.03,   // 30 cm²
			"inertia": 2.5e-4, // Beam moment of inertia
		},
	}
	model.Elements["e3"] = &core.Element{ // Right column
		ID:       "e3",
		Type:     core.ElementFrame2D,
		Nodes:    []string{"n3", "n4"},
		Material: "steel",
		Properties: map[string]float64{
			"area":    0.02,     // 20 cm²
			"inertia": 1.667e-4, // Column moment of inertia
		},
	}

	// Add fixed constraints at base
	model.Constraints["c1"] = &core.Constraint{
		ID:   "c1",
		Node: "n1",
		Type: core.ConstraintFixed,
		DOF:  []string{"Ux", "Uy", "Rz"},
	}
	model.Constraints["c2"] = &core.Constraint{
		ID:   "c2",
		Node: "n4",
		Type: core.ConstraintFixed,
		DOF:  []string{"Ux", "Uy", "Rz"},
	}

	// Add loads
	windLoad := 25.0
	deadLoad := 100.0
	if len(loads) >= 2 {
		windLoad = loads[0]
		deadLoad = loads[1]
	} else if len(loads) >= 1 {
		deadLoad = loads[0]
	}

	model.Loads["l1"] = &core.Load{ // Horizontal wind load
		ID:        "l1",
		Type:      core.LoadForce,
		Node:      "n2",
		Direction: "Fx",
		Magnitude: windLoad * 1000, // Convert kN to N
	}
	model.Loads["l2"] = &core.Load{ // Vertical dead load on beam
		ID:        "l2",
		Type:      core.LoadForce,
		Node:      "n3",
		Direction: "Fy",
		Magnitude: -deadLoad * 1000, // Convert kN to N
	}

	return model, nil
}

func saveModel(model *core.Model, filename string) error {
	// Create directory if needed
	dir := filepath.Dir(filename)
	if err := os.MkdirAll(dir, 0755); err != nil {
		return err
	}

	// Marshal to JSON with indentation
	data, err := json.MarshalIndent(model, "", "  ")
	if err != nil {
		return err
	}

	// Write to file
	return os.WriteFile(filename, data, 0644)
}
