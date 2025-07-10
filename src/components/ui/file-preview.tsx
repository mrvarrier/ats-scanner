import React, { useState, useEffect } from 'react';
import { Card, CardContent } from './card';
import { Button } from './button';
import { Dialog, DialogContent, DialogHeader, DialogTitle } from './dialog';
import { Badge } from './badge';
import { Tabs, TabsContent, TabsList, TabsTrigger } from './tabs';
import { ScrollArea } from './scroll-area';
import {
  FileText,
  Download,
  ZoomIn,
  ZoomOut,
  Search,
  Eye,
  Highlight,
  ChevronLeft,
  ChevronRight,
} from 'lucide-react';
import { Input } from './input';

interface FilePreviewProps {
  fileName: string;
  fileContent: string;
  fileType: string;
  keywords?: string[];
  missingKeywords?: string[];
  recommendations?: string[];
  onClose?: () => void;
}

interface HighlightedTextProps {
  text: string;
  keywords: string[];
  missingKeywords: string[];
  searchTerm?: string;
}

const HighlightedText: React.FC<HighlightedTextProps> = ({
  text,
  keywords = [],
  missingKeywords = [],
  searchTerm,
}) => {
  const highlightText = (content: string) => {
    // Create a map of all terms to highlight with their colors
    const highlightMap = new Map<string, string>();

    // Add keywords (green)
    keywords.forEach(keyword => {
      highlightMap.set(
        keyword.toLowerCase(),
        'bg-green-200 dark:bg-green-800 text-green-900 dark:text-green-100'
      );
    });

    // Add missing keywords (red) - if they appear in the text
    missingKeywords.forEach(keyword => {
      highlightMap.set(
        keyword.toLowerCase(),
        'bg-red-200 dark:bg-red-800 text-red-900 dark:text-red-100'
      );
    });

    // Add search term (blue)
    if (searchTerm?.trim()) {
      highlightMap.set(
        searchTerm.toLowerCase(),
        'bg-blue-200 dark:bg-blue-800 text-blue-900 dark:text-blue-100'
      );
    }

    // Sort by length (longest first) to avoid partial matches
    const sortedTerms = Array.from(highlightMap.keys()).sort(
      (a, b) => b.length - a.length
    );

    let result = content;

    sortedTerms.forEach(term => {
      const className = highlightMap.get(term);
      const regex = new RegExp(
        `\\b${term.replace(/[.*+?^${}()|[\]\\]/g, '\\$&')}\\b`,
        'gi'
      );
      result = result.replace(
        regex,
        `<span class="${className} px-1 rounded">${term}</span>`
      );
    });

    return result;
  };

  return (
    <div
      className="whitespace-pre-wrap font-mono text-sm leading-relaxed"
      dangerouslySetInnerHTML={{ __html: highlightText(text) }}
    />
  );
};

export const FilePreview: React.FC<FilePreviewProps> = ({
  fileName,
  fileContent,
  fileType,
  keywords = [],
  missingKeywords = [],
  recommendations = [],
  onClose,
}) => {
  const [zoom, setZoom] = useState(100);
  const [searchTerm, setSearchTerm] = useState('');
  const [currentSearchIndex, setCurrentSearchIndex] = useState(0);
  const [searchMatches, setSearchMatches] = useState<number[]>([]);
  const [activeTab, setActiveTab] = useState('content');

  useEffect(() => {
    if (searchTerm.trim()) {
      const matches: number[] = [];
      const regex = new RegExp(
        searchTerm.replace(/[.*+?^${}()|[\]\\]/g, '\\$&'),
        'gi'
      );
      let match;
      while ((match = regex.exec(fileContent)) !== null) {
        matches.push(match.index);
      }
      setSearchMatches(matches);
      setCurrentSearchIndex(0);
    } else {
      setSearchMatches([]);
      setCurrentSearchIndex(0);
    }
  }, [searchTerm, fileContent]);

  const handleZoomIn = () => setZoom(prev => Math.min(prev + 20, 200));
  const handleZoomOut = () => setZoom(prev => Math.max(prev - 20, 50));

  const handleSearchNext = () => {
    if (searchMatches.length > 0) {
      setCurrentSearchIndex(prev => (prev + 1) % searchMatches.length);
    }
  };

  const handleSearchPrev = () => {
    if (searchMatches.length > 0) {
      setCurrentSearchIndex(
        prev => (prev - 1 + searchMatches.length) % searchMatches.length
      );
    }
  };

  const downloadFile = () => {
    const blob = new Blob([fileContent], { type: 'text/plain' });
    const url = URL.createObjectURL(blob);
    const a = document.createElement('a');
    a.href = url;
    a.download = fileName;
    document.body.appendChild(a);
    a.click();
    document.body.removeChild(a);
    URL.revokeObjectURL(url);
  };

  const getFileIcon = () => {
    switch (fileType.toLowerCase()) {
      case 'pdf':
        return '📄';
      case 'docx':
      case 'doc':
        return '📝';
      case 'txt':
        return '📃';
      default:
        return '📄';
    }
  };

  return (
    <Dialog open={true} onOpenChange={onClose}>
      <DialogContent className="max-h-[90vh] max-w-6xl overflow-hidden">
        <DialogHeader>
          <DialogTitle className="flex items-center gap-2">
            <span className="text-2xl">{getFileIcon()}</span>
            {fileName}
            <Badge variant="outline">{fileType.toUpperCase()}</Badge>
          </DialogTitle>
        </DialogHeader>

        <div className="flex h-full flex-col">
          {/* Toolbar */}
          <div className="flex items-center justify-between border-b p-4">
            <div className="flex items-center gap-2">
              <Button variant="outline" size="sm" onClick={handleZoomOut}>
                <ZoomOut className="h-4 w-4" />
              </Button>
              <span className="min-w-[4rem] text-center text-sm font-medium">
                {zoom}%
              </span>
              <Button variant="outline" size="sm" onClick={handleZoomIn}>
                <ZoomIn className="h-4 w-4" />
              </Button>
            </div>

            <div className="flex items-center gap-2">
              <div className="relative">
                <Search className="absolute left-2 top-1/2 h-4 w-4 -translate-y-1/2 transform text-gray-400" />
                <Input
                  placeholder="Search in document..."
                  value={searchTerm}
                  onChange={e => setSearchTerm(e.target.value)}
                  className="w-64 pl-8"
                />
              </div>
              {searchMatches.length > 0 && (
                <div className="flex items-center gap-1">
                  <Button
                    variant="outline"
                    size="sm"
                    onClick={handleSearchPrev}
                  >
                    <ChevronLeft className="h-4 w-4" />
                  </Button>
                  <span className="text-sm text-gray-600">
                    {currentSearchIndex + 1} of {searchMatches.length}
                  </span>
                  <Button
                    variant="outline"
                    size="sm"
                    onClick={handleSearchNext}
                  >
                    <ChevronRight className="h-4 w-4" />
                  </Button>
                </div>
              )}
            </div>

            <Button variant="outline" size="sm" onClick={downloadFile}>
              <Download className="mr-2 h-4 w-4" />
              Download
            </Button>
          </div>

          {/* Content Tabs */}
          <Tabs
            value={activeTab}
            onValueChange={setActiveTab}
            className="flex flex-1 flex-col"
          >
            <TabsList className="grid w-full grid-cols-4">
              <TabsTrigger value="content">
                <FileText className="mr-2 h-4 w-4" />
                Content
              </TabsTrigger>
              <TabsTrigger value="keywords">
                <Highlight className="mr-2 h-4 w-4" />
                Keywords ({keywords.length})
              </TabsTrigger>
              <TabsTrigger value="missing">
                <Eye className="mr-2 h-4 w-4" />
                Missing ({missingKeywords.length})
              </TabsTrigger>
              <TabsTrigger value="recommendations">
                📋 Recommendations ({recommendations.length})
              </TabsTrigger>
            </TabsList>

            <TabsContent value="content" className="flex-1">
              <ScrollArea className="h-full rounded-lg border">
                <div className="p-6" style={{ fontSize: `${zoom}%` }}>
                  <HighlightedText
                    text={fileContent}
                    keywords={keywords}
                    missingKeywords={missingKeywords}
                    searchTerm={searchTerm}
                  />
                </div>
              </ScrollArea>
            </TabsContent>

            <TabsContent value="keywords" className="flex-1">
              <ScrollArea className="h-full rounded-lg border">
                <div className="p-6">
                  <h3 className="mb-4 text-lg font-semibold text-green-600 dark:text-green-400">
                    Found Keywords
                  </h3>
                  <div className="grid grid-cols-2 gap-2 md:grid-cols-3 lg:grid-cols-4">
                    {keywords.map((keyword, index) => (
                      <Badge
                        key={index}
                        className="bg-green-100 text-green-800 dark:bg-green-800 dark:text-green-100"
                      >
                        ✓ {keyword}
                      </Badge>
                    ))}
                  </div>
                  {keywords.length === 0 && (
                    <p className="italic text-gray-500">
                      No keywords identified in this document.
                    </p>
                  )}
                </div>
              </ScrollArea>
            </TabsContent>

            <TabsContent value="missing" className="flex-1">
              <ScrollArea className="h-full rounded-lg border">
                <div className="p-6">
                  <h3 className="mb-4 text-lg font-semibold text-red-600 dark:text-red-400">
                    Missing Keywords
                  </h3>
                  <div className="grid grid-cols-2 gap-2 md:grid-cols-3 lg:grid-cols-4">
                    {missingKeywords.map((keyword, index) => (
                      <Badge
                        key={index}
                        className="bg-red-100 text-red-800 dark:bg-red-800 dark:text-red-100"
                      >
                        ✗ {keyword}
                      </Badge>
                    ))}
                  </div>
                  {missingKeywords.length === 0 && (
                    <p className="italic text-gray-500">
                      No missing keywords identified.
                    </p>
                  )}
                </div>
              </ScrollArea>
            </TabsContent>

            <TabsContent value="recommendations" className="flex-1">
              <ScrollArea className="h-full rounded-lg border">
                <div className="p-6">
                  <h3 className="mb-4 text-lg font-semibold text-blue-600 dark:text-blue-400">
                    Improvement Recommendations
                  </h3>
                  <div className="space-y-3">
                    {recommendations.map((recommendation, index) => (
                      <Card
                        key={index}
                        className="border-l-4 border-l-blue-500"
                      >
                        <CardContent className="p-4">
                          <p className="text-sm">{recommendation}</p>
                        </CardContent>
                      </Card>
                    ))}
                  </div>
                  {recommendations.length === 0 && (
                    <p className="italic text-gray-500">
                      No recommendations available.
                    </p>
                  )}
                </div>
              </ScrollArea>
            </TabsContent>
          </Tabs>
        </div>
      </DialogContent>
    </Dialog>
  );
};

export default FilePreview;
