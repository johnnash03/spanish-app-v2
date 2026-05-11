import type { SVGProps } from "react";

export interface IconProps extends Omit<SVGProps<SVGSVGElement>, "stroke"> {
  size?: number;
  stroke?: number;
}

function Icon({ children, size = 20, stroke = 1.5, ...rest }: IconProps) {
  return (
    <svg
      width={size}
      height={size}
      viewBox="0 0 24 24"
      fill="none"
      stroke="currentColor"
      strokeWidth={stroke}
      strokeLinecap="round"
      strokeLinejoin="round"
      {...rest}
    >
      {children}
    </svg>
  );
}

export const IconHouse = (p: IconProps) => (
  <Icon {...p}>
    <path d="M3 11.5L12 4l9 7.5" />
    <path d="M5 10.5V20h14v-9.5" />
    <path d="M10 20v-5.5h4V20" />
  </Icon>
);

export const IconNotebook = (p: IconProps) => (
  <Icon {...p}>
    <rect x="5" y="3.5" width="14" height="17" rx="1.5" />
    <path d="M9 3.5v17" />
    <path d="M12 8h4M12 12h4" />
  </Icon>
);

export const IconArrowRight = (p: IconProps) => (
  <Icon {...p}>
    <path d="M5 12h14" />
    <path d="M13 6l6 6-6 6" />
  </Icon>
);

export const IconArrowLeft = (p: IconProps) => (
  <Icon {...p}>
    <path d="M19 12H5" />
    <path d="M11 18l-6-6 6-6" />
  </Icon>
);

export const IconCaret = (p: IconProps) => (
  <Icon {...p}>
    <path d="M6 9l6 6 6-6" />
  </Icon>
);

export const IconCheck = (p: IconProps) => (
  <Icon {...p}>
    <path d="M5 12.5l4.5 4.5L19 7" />
  </Icon>
);

export const IconX = (p: IconProps) => (
  <Icon {...p}>
    <path d="M6 6l12 12M18 6L6 18" />
  </Icon>
);

export const IconSearch = (p: IconProps) => (
  <Icon {...p}>
    <circle cx="11" cy="11" r="6.5" />
    <path d="M20 20l-3.5-3.5" />
  </Icon>
);

export const IconQuestion = (p: IconProps) => (
  <Icon {...p}>
    <circle cx="12" cy="12" r="9" />
    <path d="M9.5 9.5a2.5 2.5 0 015 0c0 1.5-2.5 2-2.5 4" />
    <circle cx="12" cy="17" r="0.5" fill="currentColor" />
  </Icon>
);

export const IconBook = (p: IconProps) => (
  <Icon {...p}>
    <path d="M4 4.5h7c1.5 0 2.5 1 2.5 2.5v13c0-1.5-1-2.5-2.5-2.5H4z" />
    <path d="M20 4.5h-7c-1.5 0-2.5 1-2.5 2.5v13c0-1.5 1-2.5 2.5-2.5h7z" />
  </Icon>
);

export const IconCards = (p: IconProps) => (
  <Icon {...p}>
    <rect x="3" y="6" width="13" height="14" rx="1.5" />
    <path d="M7 6V4.5a1.5 1.5 0 011.5-1.5h11a1.5 1.5 0 011.5 1.5v11a1.5 1.5 0 01-1.5 1.5H16" />
  </Icon>
);

export const IconLayers = (p: IconProps) => (
  <Icon {...p}>
    <path d="M12 3l9 5-9 5-9-5 9-5z" />
    <path d="M3 13l9 5 9-5" />
  </Icon>
);

export const IconLock = (p: IconProps) => (
  <Icon {...p}>
    <rect x="5" y="11" width="14" height="9" rx="1.5" />
    <path d="M8 11V8a4 4 0 018 0v3" />
  </Icon>
);

export const IconSpark = (p: IconProps) => (
  <Icon {...p}>
    <path d="M12 4v4M12 16v4M4 12h4M16 12h4M6.5 6.5l2.5 2.5M15 15l2.5 2.5M6.5 17.5L9 15M15 9l2.5-2.5" />
  </Icon>
);

export const IconPlus = (p: IconProps) => (
  <Icon {...p}>
    <path d="M12 5v14M5 12h14" />
  </Icon>
);

export const IconKey = (p: IconProps) => (
  <Icon {...p}>
    <circle cx="8" cy="14" r="4" />
    <path d="M11 12l9-9M16 7l3 3" />
  </Icon>
);

export const IconChevronRight = (p: IconProps) => (
  <Icon {...p}>
    <path d="M9 6l6 6-6 6" />
  </Icon>
);

export const IconChevronDown = (p: IconProps) => (
  <Icon {...p}>
    <path d="M6 9l6 6 6-6" />
  </Icon>
);

interface DotIconProps {
  size?: number;
  color?: string;
}

export const IconDot = ({ size = 8, color = "currentColor" }: DotIconProps) => (
  <span
    style={{
      width: size,
      height: size,
      borderRadius: 999,
      background: color,
      display: "inline-block",
      flexShrink: 0,
    }}
  />
);
