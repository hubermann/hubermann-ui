/**
 * Hubermann UI - Financial Dark Theme
 * 
 * Tema oscuro profesional para aplicaciones financieras y dashboards.
 * Inspirado en terminales de trading (Bloomberg, TradingView).
 * 
 * Características:
 * - Backgrounds con gradientes sutiles de negro/gris
 * - Borders de baja opacidad para separación visual mínima
 * - Jerarquía de texto clara pero no agresiva
 * - Colores semánticos para contexto financiero (bull/bear)
 * 
 * Uso: Este es el theme DEFAULT del sistema.
 */

module.exports = {
  name: 'financial-dark',
  
  colors: {
    // === BACKGROUNDS ===
    // Gradiente sutil de oscuro a menos oscuro para jerarquía
    bg: {
      primary: '#0A0E27',      // Fondo principal (más oscuro)
      secondary: '#141B34',    // Cards, containers
      tertiary: '#1E2846',     // Hover states, elevated elements
      elevated: '#252D47',     // Modals, popovers (encima de todo)
      input: '#1a1a1a',        // Form inputs específicamente
    },
    
    // === BORDERS ===
    // Sutiles, siguiendo filosofía de visual-language.js
    border: {
      subtle: '#1E293B',       // Casi imperceptible (dividers internos)
      default: '#334155',      // Uso normal (cards, inputs)
      emphasis: '#475569',     // Cuando querés que se note (focus, active)
    },
    
    // === TEXT ===
    // Jerarquía clara: primary (headers) → secondary (body) → tertiary (subtle)
    text: {
      primary: '#F3F4F6',      // Headers, títulos - blanco suave
      secondary: '#9CA3AF',    // Body text, párrafos - gris claro
      tertiary: '#6B7280',     // Subtle text, labels - gris medio
      muted: '#4B5563',        // Disabled, placeholders - gris oscuro
    },
    
    // === SEMANTIC (Financial context) ===
    // Verde = bullish/positivo, Rojo = bearish/negativo
    semantic: {
      bullish: {
        DEFAULT: '#10B981',    // Verde principal (emerald-500)
        light: '#34D399',      // Verde más claro (hover)
        dark: '#059669',       // Verde más oscuro (active)
        bg: 'rgba(16, 185, 129, 0.1)',    // Background sutil
        border: 'rgba(16, 185, 129, 0.3)', // Border más visible
      },
      bearish: {
        DEFAULT: '#EF4444',    // Rojo principal (red-500)
        light: '#F87171',      // Rojo más claro (hover)
        dark: '#DC2626',       // Rojo más oscuro (active)
        bg: 'rgba(239, 68, 68, 0.1)',     // Background sutil
        border: 'rgba(239, 68, 68, 0.3)',  // Border más visible
      },
      neutral: {
        DEFAULT: '#3B82F6',    // Azul (blue-500)
        light: '#60A5FA',
        dark: '#2563EB',
        bg: 'rgba(59, 130, 246, 0.1)',
        border: 'rgba(59, 130, 246, 0.3)',
      },
      warning: {
        DEFAULT: '#F59E0B',    // Amarillo/naranja (amber-500)
        light: '#FBBF24',
        dark: '#D97706',
        bg: 'rgba(245, 158, 11, 0.1)',
        border: 'rgba(245, 158, 11, 0.3)',
      },
    },
    
    // === ACCENT (Brand color) ===
    // Color de acento para botones primarios, links, etc.
    accent: {
      primary: '#3B82F6',      // Azul principal
      hover: '#2563EB',        // Azul hover
      active: '#1D4ED8',       // Azul active/pressed
      light: '#60A5FA',        // Azul más claro
      bg: 'rgba(59, 130, 246, 0.1)', // Background sutil
    },
  },
}
