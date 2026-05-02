import type { InputHTMLAttributes, ReactNode } from "react";
import { IconSearch } from "./icons";

type InputBareProps = InputHTMLAttributes<HTMLInputElement>;

export function InputBare({ className, ...rest }: InputBareProps) {
  return (
    <input
      className={["input-bare", className].filter(Boolean).join(" ")}
      {...rest}
    />
  );
}

interface SearchInputProps extends InputHTMLAttributes<HTMLInputElement> {
  icon?: ReactNode;
}

export function SearchInput({ className, icon, ...rest }: SearchInputProps) {
  return (
    <div style={{ position: "relative" }}>
      <span
        style={{
          position: "absolute",
          left: 14,
          top: "50%",
          transform: "translateY(-50%)",
          color: "var(--ink-3)",
          display: "flex",
        }}
      >
        {icon ?? <IconSearch size={16} />}
      </span>
      <input
        className={["search-input", className].filter(Boolean).join(" ")}
        {...rest}
      />
    </div>
  );
}
