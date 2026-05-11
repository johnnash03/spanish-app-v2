import type { ButtonHTMLAttributes, ReactNode } from "react";

type ButtonVariant = "primary" | "accent" | "secondary" | "ghost" | "disabled";
type ButtonSize = "sm" | "md" | "lg";

interface ButtonProps extends ButtonHTMLAttributes<HTMLButtonElement> {
  variant?: ButtonVariant;
  size?: ButtonSize;
  children: ReactNode;
}

const variantClass: Record<ButtonVariant, string> = {
  primary: "btn-primary",
  accent: "btn-accent",
  secondary: "btn-secondary",
  ghost: "btn-ghost",
  disabled: "btn-disabled",
};

const sizeClass: Record<ButtonSize, string> = {
  sm: "btn-sm",
  md: "",
  lg: "btn-lg",
};

export function Button({
  variant = "primary",
  size = "md",
  children,
  className,
  disabled,
  ...rest
}: ButtonProps) {
  const resolvedVariant = disabled ? "disabled" : variant;
  const cls = ["btn", variantClass[resolvedVariant], sizeClass[size], className]
    .filter(Boolean)
    .join(" ");

  return (
    <button className={cls} disabled={disabled} {...rest}>
      {children}
    </button>
  );
}

interface BadgeCountProps {
  children: ReactNode;
}

export function BadgeCount({ children }: BadgeCountProps) {
  return <span className="badge-count">{children}</span>;
}
