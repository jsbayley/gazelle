// Package core provides the fundamental types for structural analysis
package core

import (
	"fmt"
	"math"
)

// Units represents the unit system used in the model
type Units string

const (
	UnitsMetric   Units = "metric"   // N, m, kg, s
	UnitsImperial Units = "imperial" // lbf, ft, slug, s
	UnitsSI       Units = "SI"       // N, m, kg, s
)

// ModelInfo contains metadata about the structural model
type ModelInfo struct {
	Name        string `json:"name" yaml:"name"`
	Description string `json:"description" yaml:"description"`
	Version     string `json:"version" yaml:"version"`
	Units       Units  `json:"units" yaml:"units"`
	Author      string `json:"author,omitempty" yaml:"author,omitempty"`
	Created     string `json:"created,omitempty" yaml:"created,omitempty"`
}

// Node represents a structural node with coordinates
type Node struct {
	ID string  `json:"id" yaml:"id"`
	X  float64 `json:"x" yaml:"x"`
	Y  float64 `json:"y" yaml:"y"`
	Z  float64 `json:"z" yaml:"z"`
}

// ElementType defines the type of structural element
type ElementType string

const (
	ElementTruss2D ElementType = "Truss2D"
	ElementBeam2D  ElementType = "Beam2D"
	ElementFrame2D ElementType = "Frame2D"
)

// Element represents a structural element
type Element struct {
	ID         string             `json:"id" yaml:"id"`
	Type       ElementType        `json:"type" yaml:"type"`
	Nodes      []string           `json:"nodes" yaml:"nodes"`
	Material   string             `json:"material" yaml:"material"`
	Properties map[string]float64 `json:"properties" yaml:"properties"`
}

// MaterialType defines the constitutive model
type MaterialType string

const (
	MaterialLinearElastic MaterialType = "LinearElastic"
	MaterialSteel         MaterialType = "Steel"
	MaterialConcrete      MaterialType = "Concrete"
)

// Material represents a structural material
type Material struct {
	ID             string       `json:"id" yaml:"id"`
	Name           string       `json:"name" yaml:"name"`
	Type           MaterialType `json:"type" yaml:"type"`
	ElasticModulus float64      `json:"elastic_modulus" yaml:"elastic_modulus"`
	PoissonRatio   float64      `json:"poisson_ratio" yaml:"poisson_ratio"`
	Density        float64      `json:"density" yaml:"density"`
	YieldStrength  float64      `json:"yield_strength" yaml:"yield_strength"`
}

// LoadType defines the type of applied load
type LoadType string

const (
	LoadForce  LoadType = "Force"
	LoadMoment LoadType = "Moment"
)

// Load represents an applied load
type Load struct {
	ID        string   `json:"id" yaml:"id"`
	Type      LoadType `json:"type" yaml:"type"`
	Node      string   `json:"node" yaml:"node"`
	Direction string   `json:"direction" yaml:"direction"`
	Magnitude float64  `json:"magnitude" yaml:"magnitude"`
}

// ConstraintType defines boundary constraints
type ConstraintType string

const (
	ConstraintFixed  ConstraintType = "Fixed"
	ConstraintPinned ConstraintType = "Pinned"
)

// Constraint represents a boundary condition
type Constraint struct {
	ID   string         `json:"id" yaml:"id"`
	Type ConstraintType `json:"type" yaml:"type"`
	Node string         `json:"node" yaml:"node"`
	DOF  []string       `json:"dof" yaml:"dof"`
}

// Model represents a complete structural model
type Model struct {
	Info        ModelInfo              `json:"info" yaml:"info"`
	Nodes       map[string]*Node       `json:"nodes" yaml:"nodes"`
	Elements    map[string]*Element    `json:"elements" yaml:"elements"`
	Materials   map[string]*Material   `json:"materials" yaml:"materials"`
	Loads       map[string]*Load       `json:"loads" yaml:"loads"`
	Constraints map[string]*Constraint `json:"constraints" yaml:"constraints"`
}

// NewModel creates a new empty structural model
func NewModel(name, description string) *Model {
	return &Model{
		Info: ModelInfo{
			Name:        name,
			Description: description,
			Version:     "1.0",
			Units:       UnitsSI,
		},
		Nodes:       make(map[string]*Node),
		Elements:    make(map[string]*Element),
		Materials:   make(map[string]*Material),
		Loads:       make(map[string]*Load),
		Constraints: make(map[string]*Constraint),
	}
}

// Validate performs model validation
func (m *Model) Validate() error {
	if len(m.Nodes) == 0 {
		return fmt.Errorf("model must contain at least one node")
	}
	if len(m.Elements) == 0 {
		return fmt.Errorf("model must contain at least one element")
	}
	return nil
}

// Summary returns model statistics
func (m *Model) Summary() string {
	return fmt.Sprintf("Model: %s\nNodes: %d, Elements: %d, Materials: %d\n",
		m.Info.Name, len(m.Nodes), len(m.Elements), len(m.Materials))
}

// CalculateLength computes element length
func (m *Model) CalculateLength(element *Element) (float64, error) {
	if len(element.Nodes) != 2 {
		return 0, fmt.Errorf("length calculation requires 2 nodes")
	}

	n1, exists1 := m.Nodes[element.Nodes[0]]
	n2, exists2 := m.Nodes[element.Nodes[1]]

	if !exists1 || !exists2 {
		return 0, fmt.Errorf("referenced nodes not found")
	}

	dx := n2.X - n1.X
	dy := n2.Y - n1.Y
	dz := n2.Z - n1.Z

	return math.Sqrt(dx*dx + dy*dy + dz*dz), nil
}
