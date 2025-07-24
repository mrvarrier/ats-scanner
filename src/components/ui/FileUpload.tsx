import React, { useState, useCallback, useRef } from 'react';
import { invoke } from '@tauri-apps/api/tauri';
import { open } from '@tauri-apps/api/dialog';
import { Card, CardContent } from '@/components/ui/card';
import { Button } from '@/components/ui/button';
import { Progress } from '@/components/ui/progress';
import {
  Upload,
  FileText,
  X,
  CheckCircle,
  AlertCircle,
  RefreshCw,
} from 'lucide-react';
import { CommandResult } from '@/types/api';

interface DocumentInfo {
  id: string;
  filename: string;
  file_type: string;
  size: number;
  content: string;
}

interface FileUploadProps {
  onFileUploaded?: (_document: DocumentInfo) => void;
  onContentExtracted?: (_content: string) => void;
  accept?: string[];
  maxSize?: number; // in MB
  className?: string;
}

export function FileUpload({
  onFileUploaded,
  onContentExtracted,
  accept = ['.pdf', '.docx', '.doc', '.txt'],
  maxSize = 10,
  className = '',
}: FileUploadProps) {
  const [isDragOver, setIsDragOver] = useState(false);
  const [isProcessing, setIsProcessing] = useState(false);
  const [uploadProgress, setUploadProgress] = useState(0);
  const [uploadedFile, setUploadedFile] = useState<DocumentInfo | null>(null);
  const [error, setError] = useState<string | null>(null);
  const fileInputRef = useRef<HTMLInputElement>(null);

  const processFile = useCallback(
    async (filePath: string) => {
      const fileName =
        filePath.split('/').pop() ?? filePath.split('\\').pop() ?? 'unknown';

      // Check file extension
      const fileExt = fileName
        .toLowerCase()
        .substring(fileName.lastIndexOf('.'));
      if (!accept.includes(fileExt)) {
        setError(
          `File type ${fileExt} is not supported. Please upload: ${accept.join(', ')}`
        );
        return;
      }

      setIsProcessing(true);
      setError(null);
      setUploadProgress(20);

      try {
        const result = await invoke<CommandResult<DocumentInfo>>(
          'parse_document',
          { filePath }
        );

        setUploadProgress(80);

        if (result.success && result.data) {
          setUploadedFile(result.data);
          setUploadProgress(100);

          onFileUploaded?.(result.data);
          onContentExtracted?.(result.data.content);
        } else {
          throw new Error(result.error ?? 'Failed to process document');
        }
      } catch (err) {
        const errorMessage =
          err instanceof Error ? err.message : 'Failed to process file';
        setError(errorMessage);
        setUploadProgress(0);
      } finally {
        setIsProcessing(false);
      }
    },
    [accept, onFileUploaded, onContentExtracted]
  );

  const handleFileSelect = useCallback(async () => {
    try {
      const selected = await open({
        multiple: false,
        filters: [
          {
            name: 'Documents',
            extensions: accept.map(ext => ext.replace('.', '')),
          },
        ],
      });

      if (selected && typeof selected === 'string') {
        await processFile(selected);
      }
    } catch (_err) {
      setError('Failed to open file dialog');
    }
  }, [accept, processFile]);

  const handleDragOver = useCallback((e: React.DragEvent) => {
    e.preventDefault();
    e.stopPropagation();
    setIsDragOver(true);
  }, []);

  const handleDragLeave = useCallback((e: React.DragEvent) => {
    e.preventDefault();
    e.stopPropagation();
    setIsDragOver(false);
  }, []);

  const handleDrop = useCallback(async (e: React.DragEvent) => {
    e.preventDefault();
    e.stopPropagation();
    setIsDragOver(false);

    const files = Array.from(e.dataTransfer.files);
    if (files.length === 0) return;

    const _file = files[0];

    // For drag and drop, we need to save the file temporarily first
    // This is a limitation of Tauri - we can't directly process dropped files
    // In a real implementation, you'd need to save the file to a temp location first
    setError(
      'Drag and drop support requires file to be saved first. Please use the file picker instead.'
    );
  }, []);

  const handleRemoveFile = useCallback(() => {
    setUploadedFile(null);
    setError(null);
    setUploadProgress(0);
    if (fileInputRef.current) {
      fileInputRef.current.value = '';
    }
  }, []);

  const formatFileSize = (bytes: number): string => {
    if (bytes === 0) return '0 Bytes';
    const k = 1024;
    const sizes = ['Bytes', 'KB', 'MB', 'GB'];
    const i = Math.floor(Math.log(bytes) / Math.log(k));
    return parseFloat((bytes / Math.pow(k, i)).toFixed(2)) + ' ' + sizes[i];
  };

  return (
    <Card className={className}>
      <CardContent className="p-6">
        {/* Upload Area */}
        {!uploadedFile && (
          <div
            className={`rounded-lg border-2 border-dashed p-8 text-center transition-colors ${
              isDragOver
                ? 'border-primary bg-primary/5'
                : 'border-muted-foreground/25 hover:border-muted-foreground/50'
            } ${isProcessing ? 'pointer-events-none opacity-50' : ''}`}
            onDragOver={handleDragOver}
            onDragLeave={handleDragLeave}
            onDrop={handleDrop}
          >
            <div className="flex flex-col items-center gap-4">
              <div className="rounded-full bg-muted p-4">
                <Upload className="h-8 w-8 text-muted-foreground" />
              </div>

              <div className="space-y-2">
                <h3 className="text-lg font-semibold">Upload Resume</h3>
                <p className="text-sm text-muted-foreground">
                  Choose a file or drag and drop it here
                </p>
                <p className="text-xs text-muted-foreground">
                  Supported formats: {accept.join(', ')} (max {maxSize}MB)
                </p>
              </div>

              <Button
                onClick={handleFileSelect}
                disabled={isProcessing}
                className="mt-4"
              >
                {isProcessing ? (
                  <>
                    <RefreshCw className="mr-2 h-4 w-4 animate-spin" />
                    Processing...
                  </>
                ) : (
                  <>
                    <Upload className="mr-2 h-4 w-4" />
                    Choose File
                  </>
                )}
              </Button>
            </div>
          </div>
        )}

        {/* Processing Progress */}
        {isProcessing && (
          <div className="space-y-3">
            <div className="flex items-center gap-2">
              <RefreshCw className="h-4 w-4 animate-spin text-blue-600" />
              <span className="text-sm font-medium">
                Processing document...
              </span>
            </div>
            <Progress value={uploadProgress} className="w-full" />
            <p className="text-xs text-muted-foreground">
              Extracting text and analyzing document structure
            </p>
          </div>
        )}

        {/* Error Display */}
        {error && (
          <div className="flex items-start gap-3 rounded-lg border border-red-200 bg-red-50 p-4">
            <AlertCircle className="mt-0.5 h-5 w-5 flex-shrink-0 text-red-600" />
            <div className="flex-1">
              <h4 className="text-sm font-medium text-red-800">
                Upload Failed
              </h4>
              <p className="mt-1 text-sm text-red-700">{error}</p>
            </div>
          </div>
        )}

        {/* Uploaded File Display */}
        {uploadedFile && (
          <div className="space-y-4">
            <div className="flex items-center justify-between rounded-lg border border-green-200 bg-green-50 p-4">
              <div className="flex items-center gap-3">
                <div className="rounded-full bg-green-100 p-2">
                  <CheckCircle className="h-5 w-5 text-green-600" />
                </div>
                <div>
                  <h4 className="text-sm font-medium text-green-800">
                    File Uploaded Successfully
                  </h4>
                  <p className="text-sm text-green-700">Ready for analysis</p>
                </div>
              </div>
              <Button
                onClick={handleRemoveFile}
                variant="ghost"
                size="sm"
                className="text-green-700 hover:text-green-800"
              >
                <X className="h-4 w-4" />
              </Button>
            </div>

            {/* File Details */}
            <div className="rounded-lg border p-4">
              <div className="flex items-start gap-3">
                <FileText className="mt-0.5 h-5 w-5 flex-shrink-0 text-blue-600" />
                <div className="flex-1 space-y-2">
                  <div className="flex items-center justify-between">
                    <h4 className="text-sm font-medium">
                      {uploadedFile.filename}
                    </h4>
                    <span className="text-xs uppercase text-muted-foreground">
                      {uploadedFile.file_type}
                    </span>
                  </div>

                  <div className="grid grid-cols-2 gap-4 text-xs text-muted-foreground">
                    <div>
                      <span className="font-medium">Size:</span>{' '}
                      {formatFileSize(uploadedFile.size)}
                    </div>
                    <div>
                      <span className="font-medium">Words:</span>{' '}
                      {uploadedFile.content.split(/\s+/).length}
                    </div>
                  </div>

                  {/* Content Preview */}
                  <div className="mt-3">
                    <div className="mb-2 text-xs font-medium text-muted-foreground">
                      Content Preview:
                    </div>
                    <div className="max-h-32 overflow-y-auto rounded border bg-muted/50 p-3 text-xs">
                      {uploadedFile.content.substring(0, 300)}
                      {uploadedFile.content.length > 300 && '...'}
                    </div>
                  </div>

                  {/* Actions */}
                  <div className="flex gap-2 pt-2">
                    <Button
                      onClick={handleFileSelect}
                      variant="outline"
                      size="sm"
                    >
                      <Upload className="mr-2 h-3 w-3" />
                      Upload Different File
                    </Button>
                    <Button
                      onClick={() => onContentExtracted?.(uploadedFile.content)}
                      variant="outline"
                      size="sm"
                    >
                      <FileText className="mr-2 h-3 w-3" />
                      Use This Content
                    </Button>
                  </div>
                </div>
              </div>
            </div>
          </div>
        )}

        {/* Hidden file input for fallback */}
        <input
          ref={fileInputRef}
          type="file"
          accept={accept.join(',')}
          className="hidden"
          onChange={async e => {
            const file = e.target.files?.[0];
            if (file) {
              // Note: This won't work with Tauri directly as we need file paths
              // This is kept for potential future web version compatibility
              setError(
                'Please use the Choose File button for proper file selection'
              );
            }
          }}
        />
      </CardContent>
    </Card>
  );
}
