package cmd

import (
	"os"

	"github.com/sirupsen/logrus"
	"github.com/spf13/cobra"
	"github.com/spf13/viper"
)

var (
	cfgFile string
	verbose bool
)

var rootCmd = &cobra.Command{
	Use:   "gz",
	Short: "🦌 A Fast Engine for Structural Engineering 💨",
	Long: `🦌 Gazelle is a fast, reliable, and transparent structural analysis engine. 💨

Built with performance and safety in mind, Gazelle provides:
• Type-safe structural analysis with unit validation
• High-performance matrix operations for large models  
• Multi-format I/O (JSON, YAML, VTK)
• Comprehensive material libraries with design codes

Fast • Stable • Reliable • Transparent • Cross-platform • Extensible`,
	Version: "0.2.0",
	PersistentPreRun: func(cmd *cobra.Command, args []string) {
		if verbose {
			logrus.SetLevel(logrus.DebugLevel)
		} else {
			logrus.SetLevel(logrus.InfoLevel)
		}
	},
}

func Execute() error {
	return rootCmd.Execute()
}

func init() {
	cobra.OnInitialize(initConfig)

	rootCmd.PersistentFlags().StringVar(&cfgFile, "config", "", "config file")
	rootCmd.PersistentFlags().BoolVarP(&verbose, "verbose", "v", false, "verbose output")

	viper.BindPFlag("verbose", rootCmd.PersistentFlags().Lookup("verbose"))
}

func initConfig() {
	if cfgFile != "" {
		viper.SetConfigFile(cfgFile)
	} else {
		home, err := os.UserHomeDir()
		if err != nil {
			return
		}
		viper.AddConfigPath(home)
		viper.SetConfigType("yaml")
		viper.SetConfigName(".gazelle")
	}

	viper.AutomaticEnv()
	viper.ReadInConfig()
}
