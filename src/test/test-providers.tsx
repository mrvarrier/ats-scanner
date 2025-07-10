import React from 'react';

// Custom render function that includes providers
export const AllTheProviders: React.FC<{ children: React.ReactNode }> = ({
  children,
}) => {
  return <div>{children}</div>;
};
