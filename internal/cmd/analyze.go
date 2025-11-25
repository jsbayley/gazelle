package cmd

import (
	"encoding/json"
	"fmt"
	"os"
	"time"

	"github.com/jsbayley/gazelle/pkg/analysis"
	"github.com/jsbayley/gazelle/pkg/core"
	"github.com/sirupsen/logrus"
	"github.com/spf13/cobra"
)

var (
	analysisType  string
	solverType    string
	tolerance     float64
	maxIterations int
	outputFile    string
)

var analyzeCmd = &cobra.Command{
	Use:   "analyze [model-file]",
	Short: "Analyze a structural model",
	Long: `Perform structural analysis on a model file.

Supported analysis types:
  - static: Linear static analysis (default)
  - modal: Modal analysis for natural frequencies
  - dynamic: Time-history dynamic analysis

Examples:
  gz analyze model.json
  gz analyze beam.json --type modal --output results.json
  gz analyze frame.json --solver cholesky --tolerance 1e-12`,
	Args: cobra.ExactArgs(1),
	RunE: runAnalyze,
}

func init() {
	analyzeCmd.Flags().StringVarP(&analysisType, "type", "t", "static", "analysis type (static, modal, dynamic)")
	analyzeCmd.Flags().StringVarP(&solverType, "solver", "s", "auto", "solver type (auto, cholesky, lu)")
	analyzeCmd.Flags().Float64Var(&tolerance, "tolerance", 1e-9, "convergence tolerance")
	analyzeCmd.Flags().IntVar(&maxIterations, "max-iterations", 1000, "maximum iterations")
	analyzeCmd.Flags().StringVarP(&outputFile, "output", "o", "", "output file for results")

	rootCmd.AddCommand(analyzeCmd)
}

func runAnalyze(cmd *cobra.Command, args []string) error {
	modelFile := args[0]

	logrus.Infof("Loading model: %s", modelFile)

	// Load model
	model, err := loadModel(modelFile)
	if err != nil {
		return fmt.Errorf("failed to load model: %w", err)
	}

	// Validate model
	if err := model.Validate(); err != nil {
		return fmt.Errorf("model validation failed: %w", err)
	}

	logrus.Infof("Model loaded: %s", model.Info.Name)
	fmt.Printf("🦌 Gazelle Structural Analysis Engine 💨\n")
	fmt.Printf("=========================================\n")
	fmt.Printf("%s", model.Summary())

	// Create analyzer
	analyzer := &analysis.Analyzer{
		SolverType:    solverType,
		Tolerance:     tolerance,
		MaxIterations: maxIterations,
	}

	// Run analysis
	start := time.Now()
	var results *analysis.Results

	switch analysisType {
	case "static":
		logrus.Info("Running static analysis")
		results, err = analyzer.StaticAnalysis(model)
	case "modal":
		logrus.Info("Running modal analysis")
		results, err = analyzer.ModalAnalysis(model)
	case "dynamic":
		return fmt.Errorf("dynamic analysis not yet implemented")
	default:
		return fmt.Errorf("unknown analysis type: %s", analysisType)
	}

	duration := time.Since(start)

	if err != nil {
		return fmt.Errorf("analysis failed: %w", err)
	}

	logrus.Infof("Analysis completed in %v", duration)

	// Print results summary
	printAnalysisSummary(results, duration)

	// Save results if output specified
	if outputFile != "" {
		if err := saveResults(results, outputFile); err != nil {
			return fmt.Errorf("failed to save results: %w", err)
		}
		fmt.Printf("✓ Results saved to: %s\n", outputFile)
	}

	return nil
}

func loadModel(filename string) (*core.Model, error) {
	data, err := os.ReadFile(filename)
	if err != nil {
		return nil, err
	}

	var model core.Model
	if err := json.Unmarshal(data, &model); err != nil {
		return nil, err
	}

	return &model, nil
}

func printAnalysisSummary(results *analysis.Results, duration time.Duration) {
	fmt.Printf("\n📊 Analysis Results Summary\n")
	fmt.Printf("==========================\n")
	fmt.Printf("Analysis Type: %s\n", results.Type)
	fmt.Printf("Analysis Time: %v\n", duration)
	fmt.Printf("Converged: %t\n", results.Converged)

	if results.MaxDisplacement > 0 {
		fmt.Printf("Max Displacement: %.6e m\n", results.MaxDisplacement)
	}

	if results.MaxReaction > 0 {
		fmt.Printf("Max Reaction: %.6e N\n", results.MaxReaction)
	}

	if len(results.Frequencies) > 0 {
		fmt.Printf("Natural Frequencies (Hz):\n")
		for i, freq := range results.Frequencies {
			if i < 5 { // Show first 5 modes
				fmt.Printf("  Mode %d: %.3f Hz\n", i+1, freq)
			}
		}
		if len(results.Frequencies) > 5 {
			fmt.Printf("  ... and %d more modes\n", len(results.Frequencies)-5)
		}
	}

	if results.StrainEnergy > 0 {
		fmt.Printf("Strain Energy: %.6e J\n", results.StrainEnergy)
	}

	fmt.Printf("\n")
}

func saveResults(results *analysis.Results, filename string) error {
	data, err := json.MarshalIndent(results, "", "  ")
	if err != nil {
		return err
	}
	return os.WriteFile(filename, data, 0644)
}
