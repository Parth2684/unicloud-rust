"use client"
import { Button } from "./ui/button";
import { cn } from "../lib/utils";
import { ButtonHTMLAttributes, ComponentProps, FC, memo } from "react";
import { Spinner } from "./ui/spinner";

interface CustomButtonProps extends ButtonHTMLAttributes<HTMLButtonElement> {
  isLoading?: boolean;
  buttonClassName?: string;
  spinnerProps?: ComponentProps<"svg">;
}

function CustomButton({
  isLoading = false,
  buttonClassName,
  spinnerProps,
  children,
  disabled,
  ...props
}: CustomButtonProps) {
  return (
    <Button
      className={cn("relative", buttonClassName)}
      disabled={isLoading || disabled}
      {...props}
    >
      {isLoading && (
        <Spinner
          className={cn(
            "absolute h-4 w-4 animate-spin",
            spinnerProps?.className
          )}
          {...spinnerProps}
        />
      )}

      {/* hide content while loading */}
      <span className={isLoading ? "opacity-0" : "opacity-100"}>
        {children}
      </span>
    </Button>
  );
}

export default memo(CustomButton)
