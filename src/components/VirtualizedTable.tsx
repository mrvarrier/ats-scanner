import React, {
  useMemo,
  useState,
  useCallback,
  useRef,
  useEffect,
} from 'react';
import { FixedSizeList as List, VariableSizeList } from 'react-window';
import { ChevronUp, ChevronDown, Search, Filter } from 'lucide-react';
import { Button } from './ui/button';
import { Input } from './ui/input';

export interface Column<T> {
  key: keyof T;
  title: string;
  width?: number;
  minWidth?: number;
  maxWidth?: number;
  sortable?: boolean;
  filterable?: boolean;
  render?: (value: any, item: T, index: number) => React.ReactNode;
  sortFn?: (a: T, b: T) => number;
  filterFn?: (item: T, searchTerm: string) => boolean;
}

export interface VirtualizedTableProps<T> {
  data: T[];
  columns: Column<T>[];
  height?: number;
  itemHeight?: number;
  overscan?: number;
  onRowClick?: (item: T, index: number) => void;
  onRowDoubleClick?: (item: T, index: number) => void;
  loading?: boolean;
  emptyMessage?: string;
  className?: string;
  headerClassName?: string;
  rowClassName?: string | ((item: T, index: number) => string);
  stickyHeader?: boolean;
  sortable?: boolean;
  filterable?: boolean;
  selectable?: boolean;
  selectedItems?: T[];
  onSelectionChange?: (selectedItems: T[]) => void;
  getItemKey?: (item: T, index: number) => string;
}

export function VirtualizedTable<T extends Record<string, any>>({
  data,
  columns,
  height = 400,
  itemHeight = 50,
  overscan = 5,
  onRowClick,
  onRowDoubleClick,
  loading = false,
  emptyMessage = 'No data available',
  className = '',
  headerClassName = '',
  rowClassName = '',
  stickyHeader = true,
  sortable = true,
  filterable = true,
  selectable = false,
  selectedItems = [],
  onSelectionChange,
  getItemKey,
}: VirtualizedTableProps<T>) {
  const [sortConfig, setSortConfig] = useState<{
    key: keyof T | null;
    direction: 'asc' | 'desc';
  }>({ key: null, direction: 'asc' });

  const [filters, setFilters] = useState<Record<string, string>>({});
  const [globalFilter, setGlobalFilter] = useState('');

  const listRef = useRef<List>(null);

  // Process data: filter, sort
  const processedData = useMemo(() => {
    let result = [...data];

    // Apply column filters
    Object.entries(filters).forEach(([columnKey, filterValue]) => {
      if (filterValue) {
        const column = columns.find(col => col.key === columnKey);
        if (column?.filterFn) {
          result = result.filter(item => column.filterFn!(item, filterValue));
        } else {
          result = result.filter(item => {
            const value = item[columnKey];
            return String(value)
              .toLowerCase()
              .includes(filterValue.toLowerCase());
          });
        }
      }
    });

    // Apply global filter
    if (globalFilter) {
      result = result.filter(item =>
        columns.some(column => {
          const value = item[column.key];
          return String(value)
            .toLowerCase()
            .includes(globalFilter.toLowerCase());
        })
      );
    }

    // Apply sorting
    if (sortConfig.key) {
      const column = columns.find(col => col.key === sortConfig.key);
      result.sort((a, b) => {
        if (column?.sortFn) {
          return sortConfig.direction === 'asc'
            ? column.sortFn(a, b)
            : column.sortFn(b, a);
        }

        const aValue = a[sortConfig.key!];
        const bValue = b[sortConfig.key!];

        if (aValue < bValue) return sortConfig.direction === 'asc' ? -1 : 1;
        if (aValue > bValue) return sortConfig.direction === 'asc' ? 1 : -1;
        return 0;
      });
    }

    return result;
  }, [data, columns, filters, globalFilter, sortConfig]);

  const handleSort = useCallback((columnKey: keyof T) => {
    setSortConfig(prev => ({
      key: columnKey,
      direction:
        prev.key === columnKey && prev.direction === 'asc' ? 'desc' : 'asc',
    }));
  }, []);

  const handleColumnFilter = useCallback((columnKey: string, value: string) => {
    setFilters(prev => ({
      ...prev,
      [columnKey]: value,
    }));
  }, []);

  const isSelected = useCallback(
    (item: T) => {
      return selectedItems.some(selected => {
        if (getItemKey) {
          return getItemKey(selected, -1) === getItemKey(item, -1);
        }
        return selected === item;
      });
    },
    [selectedItems, getItemKey]
  );

  const toggleSelection = useCallback(
    (item: T) => {
      if (!onSelectionChange) return;

      const isCurrentlySelected = isSelected(item);
      if (isCurrentlySelected) {
        onSelectionChange(
          selectedItems.filter(selected => {
            if (getItemKey) {
              return getItemKey(selected, -1) !== getItemKey(item, -1);
            }
            return selected !== item;
          })
        );
      } else {
        onSelectionChange([...selectedItems, item]);
      }
    },
    [selectedItems, onSelectionChange, isSelected, getItemKey]
  );

  const selectAll = useCallback(() => {
    if (!onSelectionChange) return;
    onSelectionChange(processedData);
  }, [processedData, onSelectionChange]);

  const deselectAll = useCallback(() => {
    if (!onSelectionChange) return;
    onSelectionChange([]);
  }, [onSelectionChange]);

  // Calculate column widths
  const totalWidth = columns.reduce((sum, col) => sum + (col.width || 150), 0);

  // Row renderer
  const Row = useCallback(
    ({ index, style }: { index: number; style: React.CSSProperties }) => {
      const item = processedData[index];
      const selected = selectable && isSelected(item);

      const rowClasses =
        typeof rowClassName === 'function'
          ? rowClassName(item, index)
          : rowClassName;

      return (
        <div
          style={style}
          className={`flex cursor-pointer items-center border-b hover:bg-gray-50 ${selected ? 'border-blue-200 bg-blue-50' : ''} ${rowClasses} `}
          onClick={() => {
            if (selectable) {
              toggleSelection(item);
            }
            onRowClick?.(item, index);
          }}
          onDoubleClick={() => onRowDoubleClick?.(item, index)}
        >
          {selectable && (
            <div className="flex w-12 justify-center">
              <input
                type="checkbox"
                checked={selected}
                onChange={() => toggleSelection(item)}
                className="rounded"
              />
            </div>
          )}

          {columns.map((column, colIndex) => {
            const value = item[column.key];
            const cellContent = column.render
              ? column.render(value, item, index)
              : String(value);

            return (
              <div
                key={String(column.key)}
                className="truncate px-4 py-2"
                style={{
                  width: column.width || 150,
                  minWidth: column.minWidth,
                  maxWidth: column.maxWidth,
                }}
                title={String(value)}
              >
                {cellContent}
              </div>
            );
          })}
        </div>
      );
    },
    [
      processedData,
      columns,
      selectable,
      isSelected,
      toggleSelection,
      onRowClick,
      onRowDoubleClick,
      rowClassName,
    ]
  );

  // Header component
  const Header = () => (
    <div
      className={`border-b bg-gray-50 ${stickyHeader ? 'sticky top-0 z-10' : ''} ${headerClassName}`}
    >
      {/* Global filter */}
      {filterable && (
        <div className="border-b bg-white p-3">
          <div className="flex items-center gap-3">
            <Search className="h-4 w-4 text-gray-400" />
            <Input
              placeholder="Search all columns..."
              value={globalFilter}
              onChange={e => setGlobalFilter(e.target.value)}
              className="flex-1"
            />
            {globalFilter && (
              <Button
                variant="outline"
                size="sm"
                onClick={() => setGlobalFilter('')}
              >
                Clear
              </Button>
            )}
          </div>
        </div>
      )}

      {/* Column headers */}
      <div className="flex items-center">
        {selectable && (
          <div className="flex w-12 justify-center p-2">
            <input
              type="checkbox"
              checked={
                selectedItems.length === processedData.length &&
                processedData.length > 0
              }
              indeterminate={
                selectedItems.length > 0 &&
                selectedItems.length < processedData.length
              }
              onChange={e => (e.target.checked ? selectAll() : deselectAll())}
              className="rounded"
            />
          </div>
        )}

        {columns.map(column => (
          <div
            key={String(column.key)}
            className="flex cursor-pointer items-center gap-2 px-4 py-3 text-left font-medium hover:bg-gray-100"
            style={{
              width: column.width || 150,
              minWidth: column.minWidth,
              maxWidth: column.maxWidth,
            }}
            onClick={() =>
              sortable && column.sortable !== false && handleSort(column.key)
            }
          >
            <span className="truncate">{column.title}</span>
            {sortable && column.sortable !== false && (
              <div className="flex flex-col">
                <ChevronUp
                  className={`h-3 w-3 ${
                    sortConfig.key === column.key &&
                    sortConfig.direction === 'asc'
                      ? 'text-blue-600'
                      : 'text-gray-300'
                  }`}
                />
                <ChevronDown
                  className={`-mt-1 h-3 w-3 ${
                    sortConfig.key === column.key &&
                    sortConfig.direction === 'desc'
                      ? 'text-blue-600'
                      : 'text-gray-300'
                  }`}
                />
              </div>
            )}
          </div>
        ))}
      </div>

      {/* Column filters */}
      {filterable && (
        <div className="flex items-center border-t">
          {selectable && <div className="w-12" />}
          {columns.map(
            column =>
              column.filterable !== false && (
                <div
                  key={`filter-${String(column.key)}`}
                  className="px-4 py-2"
                  style={{
                    width: column.width || 150,
                    minWidth: column.minWidth,
                    maxWidth: column.maxWidth,
                  }}
                >
                  <Input
                    placeholder={`Filter ${column.title}...`}
                    value={filters[String(column.key)] || ''}
                    onChange={e =>
                      handleColumnFilter(String(column.key), e.target.value)
                    }
                    className="h-8 text-xs"
                  />
                </div>
              )
          )}
        </div>
      )}
    </div>
  );

  if (loading) {
    return (
      <div className={`rounded-lg border ${className}`}>
        <Header />
        <div
          className="flex items-center justify-center"
          style={{ height: height - 100 }}
        >
          <div className="text-center">
            <div className="mx-auto mb-2 h-8 w-8 animate-spin rounded-full border-4 border-blue-600 border-t-transparent"></div>
            <p className="text-gray-500">Loading...</p>
          </div>
        </div>
      </div>
    );
  }

  if (processedData.length === 0) {
    return (
      <div className={`rounded-lg border ${className}`}>
        <Header />
        <div
          className="flex items-center justify-center"
          style={{ height: height - 100 }}
        >
          <div className="text-center">
            <Filter className="mx-auto mb-3 h-12 w-12 text-gray-300" />
            <p className="text-gray-500">{emptyMessage}</p>
            {(globalFilter || Object.values(filters).some(f => f)) && (
              <Button
                variant="outline"
                size="sm"
                onClick={() => {
                  setGlobalFilter('');
                  setFilters({});
                }}
                className="mt-2"
              >
                Clear Filters
              </Button>
            )}
          </div>
        </div>
      </div>
    );
  }

  return (
    <div className={`overflow-hidden rounded-lg border ${className}`}>
      <Header />
      <List
        ref={listRef}
        height={height - (filterable ? 150 : 100)}
        itemCount={processedData.length}
        itemSize={itemHeight}
        overscanCount={overscan}
        width={totalWidth}
      >
        {Row}
      </List>

      {/* Footer with stats */}
      <div className="flex items-center justify-between border-t bg-gray-50 px-4 py-2 text-sm text-gray-600">
        <span>
          Showing {processedData.length} of {data.length} items
          {selectedItems.length > 0 && ` (${selectedItems.length} selected)`}
        </span>
        <div className="flex gap-2">
          {(globalFilter || Object.values(filters).some(f => f)) && (
            <Button
              variant="outline"
              size="sm"
              onClick={() => {
                setGlobalFilter('');
                setFilters({});
              }}
            >
              Clear Filters
            </Button>
          )}
        </div>
      </div>
    </div>
  );
}

// Specialized component for analysis results
export interface AnalysisResult {
  id: string;
  filename: string;
  score: number;
  status: 'completed' | 'failed' | 'processing';
  createdAt: string;
  processingTime: number;
  recommendations: string[];
}

export function AnalysisResultsTable({
  results,
  onResultClick,
  ...props
}: {
  results: AnalysisResult[];
  onResultClick?: (result: AnalysisResult) => void;
} & Omit<VirtualizedTableProps<AnalysisResult>, 'data' | 'columns'>) {
  const columns: Column<AnalysisResult>[] = [
    {
      key: 'filename',
      title: 'Resume',
      width: 200,
      render: value => (
        <div className="flex items-center gap-2">
          <div className="h-2 w-2 rounded-full bg-blue-500"></div>
          <span className="font-medium">{value}</span>
        </div>
      ),
    },
    {
      key: 'score',
      title: 'Score',
      width: 100,
      render: value => (
        <div className="flex items-center gap-2">
          <div
            className={`rounded px-2 py-1 text-sm font-medium ${
              value >= 80
                ? 'bg-green-100 text-green-800'
                : value >= 60
                  ? 'bg-yellow-100 text-yellow-800'
                  : 'bg-red-100 text-red-800'
            }`}
          >
            {value.toFixed(1)}
          </div>
        </div>
      ),
      sortFn: (a, b) => a.score - b.score,
    },
    {
      key: 'status',
      title: 'Status',
      width: 120,
      render: value => (
        <span
          className={`rounded-full px-2 py-1 text-xs font-medium ${
            value === 'completed'
              ? 'bg-green-100 text-green-800'
              : value === 'failed'
                ? 'bg-red-100 text-red-800'
                : 'bg-blue-100 text-blue-800'
          }`}
        >
          {value}
        </span>
      ),
    },
    {
      key: 'processingTime',
      title: 'Processing Time',
      width: 150,
      render: value => `${(value / 1000).toFixed(1)}s`,
    },
    {
      key: 'createdAt',
      title: 'Created',
      width: 150,
      render: value => new Date(value).toLocaleString(),
    },
  ];

  return (
    <VirtualizedTable
      data={results}
      columns={columns}
      onRowClick={onResultClick}
      getItemKey={item => item.id}
      {...props}
    />
  );
}
