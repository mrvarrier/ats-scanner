import * as React from 'react';
import { cn } from '@/lib/utils';

interface SelectProps {
  value: string;
  onValueChange: (_value: string) => void;
  disabled?: boolean;
  className?: string;
  children: React.ReactNode;
}

interface SelectContentProps {
  children: React.ReactNode;
  className?: string;
}

interface SelectItemProps {
  value: string;
  children: React.ReactNode;
  className?: string;
}

interface SelectTriggerProps {
  children: React.ReactNode;
  className?: string;
}

interface SelectValueProps {
  placeholder?: string;
  className?: string;
}

export function Select({
  value: _value,
  onValueChange,
  disabled,
  className,
  children,
}: SelectProps) {
  const [isOpen, setIsOpen] = React.useState(false);

  return (
    <div className={cn('relative', className)}>
      <div onClick={() => !disabled && setIsOpen(!isOpen)}>
        {React.Children.map(children, child =>
          React.isValidElement(child) && child.type === SelectTrigger
            ? React.cloneElement(child, {
                onClick: () => !disabled && setIsOpen(!isOpen),
              })
            : null
        )}
      </div>
      {isOpen && (
        <div className="absolute left-0 right-0 top-full z-50 mt-1 rounded-md border bg-background shadow-lg">
          {React.Children.map(children, child =>
            React.isValidElement(child) && child.type === SelectContent
              ? React.cloneElement(child, {
                  onClick: (itemValue: string) => {
                    onValueChange(itemValue);
                    setIsOpen(false);
                  },
                })
              : null
          )}
        </div>
      )}
    </div>
  );
}

export function SelectTrigger({
  children,
  className,
  ...props
}: SelectTriggerProps & React.HTMLAttributes<HTMLDivElement>) {
  return (
    <div
      className={cn(
        'flex h-10 w-full cursor-pointer items-center justify-between rounded-md border border-input bg-background px-3 py-2 text-sm ring-offset-background hover:bg-accent',
        className
      )}
      {...props}
    >
      {children}
      <span className="h-4 w-4">▼</span>
    </div>
  );
}

export function SelectValue({ placeholder, className }: SelectValueProps) {
  return <span className={cn('text-sm', className)}>{placeholder}</span>;
}

export function SelectContent({
  children,
  className,
  onClick,
}: SelectContentProps & { onClick?: (_value: string) => void }) {
  return (
    <div className={cn('p-1', className)}>
      {React.Children.map(children, child =>
        React.isValidElement(child) && child.type === SelectItem
          ? React.cloneElement(child, { onClick })
          : null
      )}
    </div>
  );
}

export function SelectItem({
  value,
  children,
  className,
  onClick,
}: SelectItemProps & { onClick?: (_value: string) => void }) {
  return (
    <div
      className={cn(
        'relative flex w-full cursor-pointer select-none items-center rounded-sm px-2 py-1.5 text-sm outline-none hover:bg-accent focus:bg-accent',
        className
      )}
      onClick={() => onClick?.(value)}
    >
      {children}
    </div>
  );
}
