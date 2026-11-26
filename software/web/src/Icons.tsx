import type { ReactNode } from 'react';

/** Small outline icons in the style Obsidian uses (24-unit grid, 2px round strokes). */
function Svg({ children, className }: { children: ReactNode; className?: string }) {
  return (
    <svg
      className={`icon ${className ?? ''}`}
      width="15"
      height="15"
      viewBox="0 0 24 24"
      fill="none"
      stroke="currentColor"
      strokeWidth="2"
      strokeLinecap="round"
      strokeLinejoin="round"
      aria-hidden
    >
      {children}
    </svg>
  );
}

/** A safe: the vault on disk. */
export const VaultIcon = ({ className }: { className?: string }) => (
  <Svg className={className}>
    <rect x="3" y="4" width="18" height="15" rx="2" />
    <circle cx="12" cy="11.5" r="3" />
    <path d="M12 8.5V7M12 16v-1.5M15 11.5h1.5M7.5 11.5H9" />
    <path d="M7 19v2M17 19v2" />
  </Svg>
);

/** A pulse: memory that lives in the running engine. */
export const RuntimeIcon = ({ className }: { className?: string }) => (
  <Svg className={className}>
    <path d="M22 12h-4l-3 8L9 4l-3 8H2" />
  </Svg>
);
