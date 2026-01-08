/**
 * Hubermann UI - Visual Language
 * 
 * Este archivo define el lenguaje visual base que aplica a TODOS los temas.
 * Representa la "personalidad" del design system.
 * 
 * Principios de diseño:
 * - Líneas sutiles (borders finos y de baja opacidad)
 * - Textos claros pero no grandes (14px base, jerarquía sutil)
 * - Bordes redondeados (4-8px predominantemente)
 * - Espaciado consistente (múltiplos de 8px)
 * 
 * IMPORTANTE: Este archivo NO debe cambiar frecuentemente.
 * Si necesitás ajustar colores, hacelo en themes/
 */

module.exports = {
  // Bordes sutiles - firma visual del sistema
  borders: {
    width: '1px',           // Siempre finos
    style: 'solid',
    opacity: 0.15,          // Sutiles, no agresivos
  },
  
  // Bordes redondeados - amigables pero profesionales
  radius: {
    none: '0',
    sm: '4px',              // Elementos pequeños (badges, chips)
    md: '6px',              // Default para la mayoría (cards, buttons)
    lg: '8px',              // Containers grandes (modals)
    xl: '12px',             // Elementos especiales
    full: '9999px',         // Circular (avatars, pills)
  },
  
  // Tipografía clara y no grande
  typography: {
    sizes: {
      xs: '0.75rem',        // 12px - labels muy pequeños
      sm: '0.875rem',       // 14px - tu tamaño default body ⭐
      base: '1rem',         // 16px - headers pequeños
      lg: '1.125rem',       // 18px - headers medianos
      xl: '1.25rem',        // 20px - headers importantes
      '2xl': '1.5rem',      // 24px - títulos de sección
      '3xl': '1.875rem',    // 30px - página headers (raro)
    },
    
    weights: {
      light: 300,           // Body text, sutil
      normal: 400,          // Default general
      medium: 500,          // Subtle emphasis
      semibold: 600,        // Headers, botones
      bold: 700,            // Raramente usado
    },
    
    lineHeight: {
      tight: 1.25,          // Headers apretados
      normal: 1.5,          // Body text cómodo
      relaxed: 1.75,        // Párrafos largos
    },
    
    letterSpacing: {
      tight: '-0.025em',
      normal: '0',
      wide: '0.025em',
    },
  },
  
  // Sistema de espaciado consistente (base 8px)
  spacing: {
    '0': '0',
    '0.5': '0.125rem',      // 2px
    '1': '0.25rem',         // 4px
    '1.5': '0.375rem',      // 6px
    '2': '0.5rem',          // 8px  - xs ⭐
    '3': '0.75rem',         // 12px - sm ⭐
    '4': '1rem',            // 16px - md ⭐ (default)
    '5': '1.25rem',         // 20px
    '6': '1.5rem',          // 24px - lg ⭐
    '8': '2rem',            // 32px - xl ⭐
    '10': '2.5rem',         // 40px
    '12': '3rem',           // 48px
    '16': '4rem',           // 64px
  },
  
  // Sombras sutiles (opcional, para elevación)
  shadows: {
    none: 'none',
    sm: '0 1px 2px 0 rgba(0, 0, 0, 0.05)',
    md: '0 4px 6px -1px rgba(0, 0, 0, 0.1)',
    lg: '0 10px 15px -3px rgba(0, 0, 0, 0.1)',
  },
  
  // Transiciones consistentes
  transitions: {
    fast: '150ms',
    normal: '200ms',
    slow: '300ms',
  },
}
