/**
 * Hubermann UI - Tailwind Base Configuration
 * 
 * Este archivo consume los design tokens y genera la configuración de Tailwind.
 * 
 * USO EN PROYECTOS:
 * 
 * // tailwind.config.js (en tu proyecto)
 * const baseConfig = require('hubermann-ui/tailwind.config.base');
 * module.exports = baseConfig;
 * 
 * // O con overrides:
 * module.exports = {
 *   ...baseConfig,
 *   theme: {
 *     ...baseConfig.theme,
 *     extend: {
 *       ...baseConfig.theme.extend,
 *       colors: {
 *         ...baseConfig.theme.extend.colors,
 *         // Tu color custom específico
 *         'my-special': '#FF00FF',
 *       }
 *     }
 *   }
 * }
 * 
 * CAMBIAR THEME:
 * Simplemente cambiá el require del theme en la línea 35
 */

const visualLanguage = require('./design-tokens/visual-language');
const theme = require('./themes/financial-dark'); // ← Cambiar acá para otro theme

module.exports = {
  content: [
    './src/**/*.rs',        // Rust files (Yew/Leptos)
    './index.html',
    './templates/**/*.html',
  ],
  
  darkMode: 'class', // Enable dark mode via class
  
  theme: {
    extend: {
      // === COLORS ===
      colors: {
        // Backgrounds
        'bg-primary': theme.colors.bg.primary,
        'bg-secondary': theme.colors.bg.secondary,
        'bg-tertiary': theme.colors.bg.tertiary,
        'bg-elevated': theme.colors.bg.elevated,
        'bg-input': theme.colors.bg.input,
        
        // Borders
        'border-subtle': theme.colors.border.subtle,
        'border-default': theme.colors.border.default,
        'border-emphasis': theme.colors.border.emphasis,
        
        // Text
        'text-primary': theme.colors.text.primary,
        'text-secondary': theme.colors.text.secondary,
        'text-tertiary': theme.colors.text.tertiary,
        'text-muted': theme.colors.text.muted,
        
        // Semantic - Bullish
        bullish: theme.colors.semantic.bullish.DEFAULT,
        'bullish-light': theme.colors.semantic.bullish.light,
        'bullish-dark': theme.colors.semantic.bullish.dark,
        
        // Semantic - Bearish
        bearish: theme.colors.semantic.bearish.DEFAULT,
        'bearish-light': theme.colors.semantic.bearish.light,
        'bearish-dark': theme.colors.semantic.bearish.dark,
        
        // Semantic - Neutral
        neutral: theme.colors.semantic.neutral.DEFAULT,
        'neutral-light': theme.colors.semantic.neutral.light,
        'neutral-dark': theme.colors.semantic.neutral.dark,
        
        // Semantic - Warning
        warning: theme.colors.semantic.warning.DEFAULT,
        'warning-light': theme.colors.semantic.warning.light,
        'warning-dark': theme.colors.semantic.warning.dark,
        
        // Accent
        'accent': theme.colors.accent.primary,
        'accent-hover': theme.colors.accent.hover,
        'accent-active': theme.colors.accent.active,
      },
      
      // === TYPOGRAPHY ===
      fontSize: visualLanguage.typography.sizes,
      fontWeight: visualLanguage.typography.weights,
      lineHeight: visualLanguage.typography.lineHeight,
      letterSpacing: visualLanguage.typography.letterSpacing,
      
      // === SPACING ===
      spacing: visualLanguage.spacing,
      
      // === BORDERS ===
      borderRadius: visualLanguage.radius,
      borderWidth: {
        DEFAULT: visualLanguage.borders.width,
      },
      
      // === SHADOWS ===
      boxShadow: visualLanguage.shadows,
      
      // === TRANSITIONS ===
      transitionDuration: {
        fast: visualLanguage.transitions.fast,
        DEFAULT: visualLanguage.transitions.normal,
        slow: visualLanguage.transitions.slow,
      },
      
      // === FONTS ===
      fontFamily: {
        sans: ['Inter', 'system-ui', '-apple-system', 'sans-serif'],
        mono: ['JetBrains Mono', 'Menlo', 'Monaco', 'monospace'],
      },
    },
  },
  
  plugins: [],
}
