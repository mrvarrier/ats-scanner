import React, { useState, useCallback, useMemo } from 'react';

export type ValidationRule<T> = {
  required?: boolean;
  minLength?: number;
  maxLength?: number;
  pattern?: RegExp;
  custom?: (_value: T) => string | null;
  message?: string;
};

export type ValidationRules<T> = {
  [_K in keyof T]?: ValidationRule<T[_K]>;
};

export type ValidationErrors<T> = {
  [_K in keyof T]?: string;
};

export type FormState<T> = {
  values: T;
  errors: ValidationErrors<T>;
  touched: { [_K in keyof T]?: boolean };
  isValid: boolean;
  isSubmitting: boolean;
};

export function useFormValidation<T extends Record<string, unknown>>(
  initialValues: T,
  validationRules: ValidationRules<T> = {}
) {
  const [values, setValues] = useState<T>(initialValues);
  const [errors, setErrors] = useState<ValidationErrors<T>>({});
  const [touched, setTouched] = useState<{ [_K in keyof T]?: boolean }>({});
  const [isSubmitting, setIsSubmitting] = useState(false);

  // Validate a single field
  const validateField = useCallback(
    (name: keyof T, value: unknown): string | null => {
      const rules = validationRules[name];
      if (!rules) return null;

      // Required validation
      if (
        rules.required &&
        (!value || (typeof value === 'string' && !value.trim()))
      ) {
        return rules.message ?? `${String(name)} is required`;
      }

      // Skip other validations if field is empty and not required
      if (!value && !rules.required) return null;

      // String-specific validations
      if (typeof value === 'string') {
        // Min length validation
        if (rules.minLength && value.length < rules.minLength) {
          return (
            rules.message ??
            `${String(name)} must be at least ${rules.minLength} characters`
          );
        }

        // Max length validation
        if (rules.maxLength && value.length > rules.maxLength) {
          return (
            rules.message ??
            `${String(name)} must be no more than ${rules.maxLength} characters`
          );
        }

        // Pattern validation
        if (rules.pattern && !rules.pattern.test(value)) {
          return rules.message ?? `${String(name)} format is invalid`;
        }
      }

      // Custom validation
      if (rules.custom) {
        return rules.custom(value as T[keyof T]);
      }

      return null;
    },
    [validationRules]
  );

  // Validate all fields
  const validateForm = useCallback((): ValidationErrors<T> => {
    const newErrors: ValidationErrors<T> = {};

    Object.keys(values).forEach(key => {
      const error = validateField(key as keyof T, values[key as keyof T]);
      if (error) {
        newErrors[key as keyof T] = error;
      }
    });

    return newErrors;
  }, [values, validateField]);

  // Check if form is valid
  const isValid = useMemo(() => {
    const currentErrors = validateForm();
    return Object.keys(currentErrors).length === 0;
  }, [validateForm]);

  // Set field value
  const setValue = useCallback(
    (name: keyof T, value: unknown) => {
      setValues(prev => ({ ...prev, [name]: value }));

      // Validate field if it has been touched
      if (touched[name]) {
        const error = validateField(name, value);
        setErrors(prev => ({
          ...prev,
          [name]: error ?? undefined,
        }));
      }
    },
    [touched, validateField]
  );

  // Set multiple values
  const setValues_ = useCallback((newValues: Partial<T>) => {
    setValues(prev => ({ ...prev, ...newValues }));
  }, []);

  // Handle field blur (mark as touched and validate)
  const handleBlur = useCallback(
    (name: keyof T) => {
      setTouched(prev => ({ ...prev, [name]: true }));

      const error = validateField(name, values[name]);
      setErrors(prev => ({
        ...prev,
        [name]: error ?? undefined,
      }));
    },
    [values, validateField]
  );

  // Handle field change
  const handleChange = useCallback(
    (name: keyof T, value: unknown) => {
      setValue(name, value);
    },
    [setValue]
  );

  // Reset form
  const reset = useCallback(() => {
    setValues(initialValues);
    setErrors({});
    setTouched({});
    setIsSubmitting(false);
  }, [initialValues]);

  // Submit form
  const handleSubmit = useCallback(
    async (onSubmit: (_values: T) => Promise<void> | void) => {
      // Mark all fields as touched
      const allTouched = Object.keys(values).reduce(
        (acc, key) => ({
          ...acc,
          [key]: true,
        }),
        {}
      );
      setTouched(allTouched);

      // Validate all fields
      const formErrors = validateForm();
      setErrors(formErrors);

      // Don't submit if there are errors
      if (Object.keys(formErrors).length > 0) {
        return false;
      }

      setIsSubmitting(true);
      try {
        await onSubmit(values);
        return true;
      } catch {
        // Form submission failed silently
        return false;
      } finally {
        setIsSubmitting(false);
      }
    },
    [values, validateForm]
  );

  // Get field props for easy integration with form controls
  const getFieldProps = useCallback(
    (name: keyof T) => ({
      value: values[name] || '',
      onChange: (
        e: React.ChangeEvent<
          HTMLInputElement | HTMLTextAreaElement | HTMLSelectElement
        >
      ) => {
        handleChange(name, e.target.value);
      },
      onBlur: () => handleBlur(name),
      error: errors[name],
      'aria-invalid': !!errors[name],
      'aria-describedby': errors[name] ? `${String(name)}-error` : undefined,
    }),
    [values, errors, handleChange, handleBlur]
  );

  // Get field state
  const getFieldState = useCallback(
    (name: keyof T) => ({
      value: values[name],
      error: errors[name],
      touched: touched[name],
      hasError: !!errors[name],
      isValid: !errors[name],
    }),
    [values, errors, touched]
  );

  return {
    values,
    errors,
    touched,
    isValid,
    isSubmitting,
    setValue,
    setValues: setValues_,
    handleChange,
    handleBlur,
    handleSubmit,
    reset,
    validateField,
    validateForm,
    getFieldProps,
    getFieldState,
  };
}

// Common validation rules
export const commonValidationRules = {
  email: {
    pattern: /^[^\s@]+@[^\s@]+\.[^\s@]+$/,
    message: 'Please enter a valid email address',
  },

  phone: {
    pattern: /^[+]?[1-9][\d]{0,15}$/,
    message: 'Please enter a valid phone number',
  },

  url: {
    pattern: /^https?:\/\/.+/,
    message: 'Please enter a valid URL',
  },

  strongPassword: {
    pattern:
      /^(?=.*[a-z])(?=.*[A-Z])(?=.*\d)(?=.*[@$!%*?&])[A-Za-z\d@$!%*?&]{8,}$/,
    message:
      'Password must be at least 8 characters with uppercase, lowercase, number, and special character',
  },

  alphanumeric: {
    pattern: /^[a-zA-Z0-9]+$/,
    message: 'Only letters and numbers are allowed',
  },

  noSpaces: {
    pattern: /^\S+$/,
    message: 'Spaces are not allowed',
  },
};

// Validation helpers
export const validators = {
  required: (message?: string) => ({
    required: true,
    message: message ?? 'This field is required',
  }),

  minLength: (min: number, message?: string) => ({
    minLength: min,
    message: message ?? `Must be at least ${min} characters`,
  }),

  maxLength: (max: number, message?: string) => ({
    maxLength: max,
    message: message ?? `Must be no more than ${max} characters`,
  }),

  pattern: (pattern: RegExp, message: string) => ({
    pattern,
    message,
  }),

  custom: (validator: (_value: unknown) => string | null) => ({
    custom: validator,
  }),

  // Specific validators
  email: (message?: string) => ({
    ...commonValidationRules.email,
    message: message ?? commonValidationRules.email.message,
  }),

  phone: (message?: string) => ({
    ...commonValidationRules.phone,
    message: message ?? commonValidationRules.phone.message,
  }),

  url: (message?: string) => ({
    ...commonValidationRules.url,
    message: message ?? commonValidationRules.url.message,
  }),

  fileSize: (maxSizeMB: number) => ({
    custom: (file: File) => {
      if (!file) return null;
      const maxSizeBytes = maxSizeMB * 1024 * 1024;
      return file.size > maxSizeBytes
        ? `File size must be less than ${maxSizeMB}MB`
        : null;
    },
  }),

  fileType: (allowedTypes: string[]) => ({
    custom: (file: File) => {
      if (!file) return null;
      return allowedTypes.includes(file.type)
        ? null
        : `File type must be one of: ${allowedTypes.join(', ')}`;
    },
  }),

  confirmPassword: (passwordField: string) => ({
    custom: (value: string, allValues: Record<string, unknown>) => {
      return value === allValues[passwordField]
        ? null
        : 'Passwords do not match';
    },
  }),
};
