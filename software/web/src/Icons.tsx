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

/** Reading used: a check in a circle. */
export const UsedIcon = ({ className }: { className?: string }) => (
  <Svg className={className}>
    <circle cx="12" cy="12" r="9" />
    <path d="M8 12.5l2.7 2.7L16 9.8" />
  </Svg>
);

/** Reading matched but not used: a dashed circle. */
export const UnusedIcon = ({ className }: { className?: string }) => (
  <Svg className={className}>
    <circle cx="12" cy="12" r="9" strokeDasharray="3.2 3.2" />
  </Svg>
);

/** Reading failed: a cross in a circle. */
export const FailedIcon = ({ className }: { className?: string }) => (
  <Svg className={className}>
    <circle cx="12" cy="12" r="9" />
    <path d="M9.5 9.5l5 5M14.5 9.5l-5 5" />
  </Svg>
);

/** A pulse: memory that lives in the running engine. */
export const RuntimeIcon = ({ className }: { className?: string }) => (
  <Svg className={className}>
    <path d="M22 12h-4l-3 8L9 4l-3 8H2" />
  </Svg>
);

export const MicIcon = ({ className }: { className?: string }) => (
  <Svg className={className}>
    <rect x="9" y="3" width="6" height="11" rx="3" />
    <path d="M5 11a7 7 0 0 0 14 0M12 18v3" />
  </Svg>
);

export const KeyboardIcon = ({ className }: { className?: string }) => (
  <Svg className={className}>
    <rect x="2.5" y="6" width="19" height="12" rx="2" />
    <path d="M6.5 10h.01M10 10h.01M13.5 10h.01M17.5 10h.01M7.5 14h9" />
  </Svg>
);
