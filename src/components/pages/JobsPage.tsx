import { useState, useEffect, useCallback } from 'react';
import { invoke } from '@tauri-apps/api/tauri';
import {
  Card,
  CardContent,
  CardDescription,
  CardHeader,
  CardTitle,
} from '@/components/ui/card';
import { Button } from '@/components/ui/button';
import { Input } from '@/components/ui/input';
import { Badge } from '@/components/ui/badge';
import { Separator } from '@/components/ui/separator';
import { Label } from '@/components/ui/label';
import { toast } from '@/hooks/use-toast';
import {
  Plus,
  Search,
  Filter,
  Edit,
  Trash2,
  Eye,
  ExternalLink,
  Building2,
  MapPin,
  DollarSign,
  Briefcase,
  Copy,
  Archive,
  ArchiveRestore,
} from 'lucide-react';
import type {
  CommandResult,
  JobDescription,
  JobStatus,
  JobPriority,
  ApplicationStatus,
  JobSortOption,
  SortOrder,
} from '@/types';

const JOB_STATUS_COLORS: Record<JobStatus, string> = {
  Draft: 'bg-gray-100 text-gray-800',
  Active: 'bg-blue-100 text-blue-800',
  Applied: 'bg-yellow-100 text-yellow-800',
  Interviewing: 'bg-purple-100 text-purple-800',
  Offered: 'bg-green-100 text-green-800',
  Rejected: 'bg-red-100 text-red-800',
  Withdrawn: 'bg-orange-100 text-orange-800',
  Closed: 'bg-gray-100 text-gray-800',
};

const PRIORITY_COLORS: Record<JobPriority, string> = {
  Low: 'bg-gray-100 text-gray-800',
  Medium: 'bg-blue-100 text-blue-800',
  High: 'bg-orange-100 text-orange-800',
  Critical: 'bg-red-100 text-red-800',
};

const APPLICATION_STATUS_COLORS: Record<ApplicationStatus, string> = {
  NotApplied: 'bg-gray-100 text-gray-800',
  Applied: 'bg-blue-100 text-blue-800',
  ApplicationReviewed: 'bg-indigo-100 text-indigo-800',
  PhoneScreen: 'bg-purple-100 text-purple-800',
  TechnicalInterview: 'bg-violet-100 text-violet-800',
  OnSiteInterview: 'bg-pink-100 text-pink-800',
  FinalRound: 'bg-rose-100 text-rose-800',
  OfferReceived: 'bg-green-100 text-green-800',
  OfferAccepted: 'bg-emerald-100 text-emerald-800',
  OfferDeclined: 'bg-red-100 text-red-800',
  Rejected: 'bg-red-100 text-red-800',
  Withdrawn: 'bg-orange-100 text-orange-800',
};

export function JobsPage() {
  const [jobs, setJobs] = useState<JobDescription[]>([]);
  const [filteredJobs, setFilteredJobs] = useState<JobDescription[]>([]);
  const [isLoading, setIsLoading] = useState(true);
  const [searchQuery, setSearchQuery] = useState('');
  const [selectedJob, setSelectedJob] = useState<JobDescription | null>(null);
  const [isCreateMode, setIsCreateMode] = useState(false);
  const [isEditMode, setIsEditMode] = useState(false);
  const [showFilters, setShowFilters] = useState(false);
  const [includeArchived, setIncludeArchived] = useState(false);

  // Filter states
  const [companyFilter, setCompanyFilter] = useState('');
  const [locationFilter, setLocationFilter] = useState('');
  const [statusFilter, setStatusFilter] = useState<JobStatus | ''>('');
  const [priorityFilter, setPriorityFilter] = useState<JobPriority | ''>('');
  const [applicationStatusFilter, setApplicationStatusFilter] = useState<
    ApplicationStatus | ''
  >('');
  const [sortBy, setSortBy] = useState<JobSortOption>('UpdatedAt');
  const [sortOrder, setSortOrder] = useState<SortOrder>('Desc');

  // Load jobs
  const loadJobs = useCallback(async () => {
    setIsLoading(true);
    try {
      const result = await invoke<CommandResult<JobDescription[]>>(
        'get_job_descriptions',
        {
          includeArchived: includeArchived,
        }
      );

      if (result.success && result.data) {
        setJobs(result.data);
        setFilteredJobs(result.data);
      } else {
        toast({
          title: 'Error loading jobs',
          description: result.error ?? 'Failed to load job descriptions',
          variant: 'destructive',
        });
      }
    } catch (error) {
      console.error('Error loading jobs:', error);
      toast({
        title: 'Error loading jobs',
        description: 'An unexpected error occurred',
        variant: 'destructive',
      });
    } finally {
      setIsLoading(false);
    }
  }, [includeArchived]);

  // Filter jobs locally
  const filterJobs = useCallback(() => {
    let filtered = [...jobs];

    if (searchQuery.trim()) {
      const query = searchQuery.toLowerCase();
      filtered = filtered.filter(
        job =>
          job.title.toLowerCase().includes(query) ||
          job.company.toLowerCase().includes(query) ||
          job.content.toLowerCase().includes(query) ||
          job.location.toLowerCase().includes(query)
      );
    }

    if (companyFilter.trim()) {
      const company = companyFilter.toLowerCase();
      filtered = filtered.filter(job =>
        job.company.toLowerCase().includes(company)
      );
    }

    if (locationFilter.trim()) {
      const location = locationFilter.toLowerCase();
      filtered = filtered.filter(job =>
        job.location.toLowerCase().includes(location)
      );
    }

    if (statusFilter) {
      filtered = filtered.filter(job => job.status === statusFilter);
    }

    if (priorityFilter) {
      filtered = filtered.filter(job => job.priority === priorityFilter);
    }

    if (applicationStatusFilter) {
      filtered = filtered.filter(
        job => job.application_status === applicationStatusFilter
      );
    }

    // Sort jobs
    filtered.sort((a, b) => {
      let aValue: string | number | Date;
      let bValue: string | number | Date;

      switch (sortBy) {
        case 'Title':
          aValue = a.title;
          bValue = b.title;
          break;
        case 'Company':
          aValue = a.company;
          bValue = b.company;
          break;
        case 'CreatedAt':
          aValue = new Date(a.created_at);
          bValue = new Date(b.created_at);
          break;
        case 'UpdatedAt':
          aValue = new Date(a.updated_at);
          bValue = new Date(b.updated_at);
          break;
        case 'PostedDate':
          aValue = a.posted_date ? new Date(a.posted_date) : new Date(0);
          bValue = b.posted_date ? new Date(b.posted_date) : new Date(0);
          break;
        case 'ApplicationDeadline':
          aValue = a.application_deadline
            ? new Date(a.application_deadline)
            : new Date(8640000000000000);
          bValue = b.application_deadline
            ? new Date(b.application_deadline)
            : new Date(8640000000000000);
          break;
        case 'SalaryMin':
          aValue = a.salary_range_min ?? 0;
          bValue = b.salary_range_min ?? 0;
          break;
        case 'SalaryMax':
          aValue = a.salary_range_max ?? 0;
          bValue = b.salary_range_max ?? 0;
          break;
        default:
          aValue = new Date(a.updated_at);
          bValue = new Date(b.updated_at);
      }

      if (sortOrder === 'Asc') {
        return aValue < bValue ? -1 : aValue > bValue ? 1 : 0;
      } else {
        return aValue > bValue ? -1 : aValue < bValue ? 1 : 0;
      }
    });

    setFilteredJobs(filtered);
  }, [
    jobs,
    searchQuery,
    companyFilter,
    locationFilter,
    statusFilter,
    priorityFilter,
    applicationStatusFilter,
    sortBy,
    sortOrder,
  ]);

  // Effects
  useEffect(() => {
    void loadJobs();
  }, [loadJobs]);

  useEffect(() => {
    filterJobs();
  }, [filterJobs]);

  // Handlers
  const handleCreateJob = () => {
    setSelectedJob(null);
    setIsCreateMode(true);
    setIsEditMode(false);
  };

  const handleEditJob = (job: JobDescription) => {
    setSelectedJob(job);
    setIsCreateMode(false);
    setIsEditMode(true);
  };

  const handleViewJob = (job: JobDescription) => {
    setSelectedJob(job);
    setIsCreateMode(false);
    setIsEditMode(false);
  };

  const handleDeleteJob = async (job: JobDescription) => {
    if (
      !confirm(
        `Are you sure you want to delete "${job.title}" at ${job.company}?`
      )
    ) {
      return;
    }

    try {
      const result = await invoke<CommandResult<string>>(
        'delete_job_description',
        {
          id: job.id,
        }
      );

      if (result.success) {
        toast({
          title: 'Job deleted',
          description: `"${job.title}" has been deleted successfully`,
        });
        void loadJobs();
      } else {
        toast({
          title: 'Error deleting job',
          description: result.error ?? 'Failed to delete job description',
          variant: 'destructive',
        });
      }
    } catch (error) {
      console.error('Error deleting job:', error);
      toast({
        title: 'Error deleting job',
        description: 'An unexpected error occurred',
        variant: 'destructive',
      });
    }
  };

  const handleDuplicateJob = (job: JobDescription) => {
    const duplicatedJob: JobDescription = {
      ...job,
      id: crypto.randomUUID(),
      title: `${job.title} (Copy)`,
      status: 'Draft',
      application_status: 'NotApplied',
      application_date: undefined,
      interview_date: undefined,
      created_at: new Date().toISOString(),
      updated_at: new Date().toISOString(),
    };
    setSelectedJob(duplicatedJob);
    setIsCreateMode(true);
    setIsEditMode(false);
  };

  const handleToggleArchive = async (job: JobDescription) => {
    const updatedJob = {
      ...job,
      is_archived: !job.is_archived,
      updated_at: new Date().toISOString(),
    };

    try {
      const result = await invoke<CommandResult<string>>(
        'update_job_description',
        {
          job: updatedJob,
        }
      );

      if (result.success) {
        toast({
          title: job.is_archived ? 'Job restored' : 'Job archived',
          description: `"${job.title}" has been ${job.is_archived ? 'restored' : 'archived'} successfully`,
        });
        void loadJobs();
      } else {
        toast({
          title: 'Error updating job',
          description: result.error ?? 'Failed to update job description',
          variant: 'destructive',
        });
      }
    } catch (error) {
      console.error('Error updating job:', error);
      toast({
        title: 'Error updating job',
        description: 'An unexpected error occurred',
        variant: 'destructive',
      });
    }
  };

  const formatSalary = (min?: number, max?: number, currency = 'USD') => {
    if (!min && !max) return 'Not specified';
    const format = (amount: number) =>
      new Intl.NumberFormat('en-US', {
        style: 'currency',
        currency,
        minimumFractionDigits: 0,
        maximumFractionDigits: 0,
      }).format(amount);

    if (min && max) return `${format(min)} - ${format(max)}`;
    if (min) return `${format(min)}+`;
    if (max) return `Up to ${format(max)}`;
    return 'Not specified';
  };

  const formatDate = (dateString?: string) => {
    if (!dateString) return 'Not set';
    return new Date(dateString).toLocaleDateString();
  };

  const clearFilters = () => {
    setSearchQuery('');
    setCompanyFilter('');
    setLocationFilter('');
    setStatusFilter('');
    setPriorityFilter('');
    setApplicationStatusFilter('');
    setSortBy('UpdatedAt');
    setSortOrder('Desc');
  };

  if (isLoading) {
    return (
      <div className="flex h-full items-center justify-center">
        <div className="text-center">
          <div className="mx-auto mb-4 h-8 w-8 animate-spin rounded-full border-4 border-primary border-t-transparent"></div>
          <p className="text-muted-foreground">Loading jobs...</p>
        </div>
      </div>
    );
  }

  // Show job form when creating or editing
  if (isCreateMode || isEditMode) {
    return (
      <JobForm
        job={selectedJob}
        isEdit={isEditMode}
        onSave={() => {
          setIsCreateMode(false);
          setIsEditMode(false);
          setSelectedJob(null);
          void loadJobs();
        }}
        onCancel={() => {
          setIsCreateMode(false);
          setIsEditMode(false);
          setSelectedJob(null);
        }}
      />
    );
  }

  // Show job details when viewing
  if (selectedJob && !isCreateMode && !isEditMode) {
    return (
      <JobDetails
        job={selectedJob}
        onEdit={() => setIsEditMode(true)}
        onDelete={() => handleDeleteJob(selectedJob)}
        onDuplicate={() => handleDuplicateJob(selectedJob)}
        onArchive={() => handleToggleArchive(selectedJob)}
        onBack={() => setSelectedJob(null)}
      />
    );
  }

  return (
    <div className="flex h-full flex-col space-y-6 p-6">
      {/* Header */}
      <div className="flex items-center justify-between">
        <div>
          <h1 className="text-3xl font-bold tracking-tight">
            Job Descriptions
          </h1>
          <p className="text-muted-foreground">
            Manage and track your job applications
          </p>
        </div>
        <div className="flex items-center space-x-2">
          <Button
            variant="outline"
            size="sm"
            onClick={() => setIncludeArchived(!includeArchived)}
          >
            {includeArchived ? (
              <Archive className="mr-2 h-4 w-4" />
            ) : (
              <ArchiveRestore className="mr-2 h-4 w-4" />
            )}
            {includeArchived ? 'Hide Archived' : 'Show Archived'}
          </Button>
          <Button onClick={handleCreateJob}>
            <Plus className="mr-2 h-4 w-4" />
            Add Job
          </Button>
        </div>
      </div>

      {/* Search and Filters */}
      <Card>
        <CardContent className="pt-6">
          <div className="flex flex-col space-y-4">
            <div className="flex items-center space-x-2">
              <div className="relative flex-1">
                <Search className="absolute left-3 top-3 h-4 w-4 text-muted-foreground" />
                <Input
                  placeholder="Search jobs by title, company, or content..."
                  value={searchQuery}
                  onChange={e => setSearchQuery(e.target.value)}
                  className="pl-10"
                />
              </div>
              <Button
                variant="outline"
                onClick={() => setShowFilters(!showFilters)}
              >
                <Filter className="mr-2 h-4 w-4" />
                Filters
              </Button>
              <Button variant="outline" onClick={clearFilters}>
                Clear
              </Button>
            </div>

            {showFilters && (
              <>
                <Separator />
                <div className="grid grid-cols-1 gap-4 md:grid-cols-3 lg:grid-cols-5">
                  <div>
                    <Label htmlFor="company-filter">Company</Label>
                    <Input
                      id="company-filter"
                      placeholder="Filter by company"
                      value={companyFilter}
                      onChange={e => setCompanyFilter(e.target.value)}
                    />
                  </div>
                  <div>
                    <Label htmlFor="location-filter">Location</Label>
                    <Input
                      id="location-filter"
                      placeholder="Filter by location"
                      value={locationFilter}
                      onChange={e => setLocationFilter(e.target.value)}
                    />
                  </div>
                  <div>
                    <Label htmlFor="status-filter">Status</Label>
                    <select
                      id="status-filter"
                      value={statusFilter}
                      onChange={e =>
                        setStatusFilter((e.target.value as JobStatus) || '')
                      }
                      className="flex h-10 w-full rounded-md border border-input bg-background px-3 py-2 text-sm ring-offset-background placeholder:text-muted-foreground focus:border-ring focus:outline-none focus:ring-2 focus:ring-ring focus:ring-offset-2"
                    >
                      <option value="">All statuses</option>
                      <option value="Draft">Draft</option>
                      <option value="Active">Active</option>
                      <option value="Applied">Applied</option>
                      <option value="Interviewing">Interviewing</option>
                      <option value="Offered">Offered</option>
                      <option value="Rejected">Rejected</option>
                      <option value="Withdrawn">Withdrawn</option>
                      <option value="Closed">Closed</option>
                    </select>
                  </div>
                  <div>
                    <Label htmlFor="priority-filter">Priority</Label>
                    <select
                      id="priority-filter"
                      value={priorityFilter}
                      onChange={e =>
                        setPriorityFilter((e.target.value as JobPriority) || '')
                      }
                      className="flex h-10 w-full rounded-md border border-input bg-background px-3 py-2 text-sm ring-offset-background placeholder:text-muted-foreground focus:border-ring focus:outline-none focus:ring-2 focus:ring-ring focus:ring-offset-2"
                    >
                      <option value="">All priorities</option>
                      <option value="Low">Low</option>
                      <option value="Medium">Medium</option>
                      <option value="High">High</option>
                      <option value="Critical">Critical</option>
                    </select>
                  </div>
                  <div>
                    <Label htmlFor="app-status-filter">
                      Application Status
                    </Label>
                    <select
                      id="app-status-filter"
                      value={applicationStatusFilter}
                      onChange={e =>
                        setApplicationStatusFilter(
                          (e.target.value as ApplicationStatus) || ''
                        )
                      }
                      className="flex h-10 w-full rounded-md border border-input bg-background px-3 py-2 text-sm ring-offset-background placeholder:text-muted-foreground focus:border-ring focus:outline-none focus:ring-2 focus:ring-ring focus:ring-offset-2"
                    >
                      <option value="">All application statuses</option>
                      <option value="NotApplied">Not Applied</option>
                      <option value="Applied">Applied</option>
                      <option value="ApplicationReviewed">
                        Application Reviewed
                      </option>
                      <option value="PhoneScreen">Phone Screen</option>
                      <option value="TechnicalInterview">
                        Technical Interview
                      </option>
                      <option value="OnSiteInterview">On-site Interview</option>
                      <option value="FinalRound">Final Round</option>
                      <option value="OfferReceived">Offer Received</option>
                      <option value="OfferAccepted">Offer Accepted</option>
                      <option value="OfferDeclined">Offer Declined</option>
                      <option value="Rejected">Rejected</option>
                      <option value="Withdrawn">Withdrawn</option>
                    </select>
                  </div>
                </div>
                <div className="flex items-center space-x-4">
                  <div>
                    <Label htmlFor="sort-by">Sort by</Label>
                    <select
                      id="sort-by"
                      value={sortBy}
                      onChange={e => setSortBy(e.target.value as JobSortOption)}
                      className="flex h-10 w-full rounded-md border border-input bg-background px-3 py-2 text-sm ring-offset-background placeholder:text-muted-foreground focus:border-ring focus:outline-none focus:ring-2 focus:ring-ring focus:ring-offset-2"
                    >
                      <option value="UpdatedAt">Last Updated</option>
                      <option value="CreatedAt">Created Date</option>
                      <option value="Title">Title</option>
                      <option value="Company">Company</option>
                      <option value="PostedDate">Posted Date</option>
                      <option value="ApplicationDeadline">
                        Application Deadline
                      </option>
                      <option value="SalaryMin">Salary (Min)</option>
                      <option value="SalaryMax">Salary (Max)</option>
                    </select>
                  </div>
                  <div>
                    <Label htmlFor="sort-order">Order</Label>
                    <select
                      id="sort-order"
                      value={sortOrder}
                      onChange={e => setSortOrder(e.target.value as SortOrder)}
                      className="flex h-10 w-full rounded-md border border-input bg-background px-3 py-2 text-sm ring-offset-background placeholder:text-muted-foreground focus:border-ring focus:outline-none focus:ring-2 focus:ring-ring focus:ring-offset-2"
                    >
                      <option value="Desc">Descending</option>
                      <option value="Asc">Ascending</option>
                    </select>
                  </div>
                </div>
              </>
            )}
          </div>
        </CardContent>
      </Card>

      {/* Jobs List */}
      {filteredJobs.length === 0 ? (
        <Card>
          <CardContent className="flex h-64 items-center justify-center">
            <div className="text-center">
              <Briefcase className="mx-auto mb-4 h-12 w-12 text-muted-foreground" />
              <h3 className="mb-2 text-lg font-semibold">No jobs found</h3>
              <p className="mb-4 text-muted-foreground">
                {searchQuery ||
                companyFilter ||
                locationFilter ||
                statusFilter ||
                priorityFilter ||
                applicationStatusFilter
                  ? 'No jobs match your current filters. Try adjusting your search criteria.'
                  : "You haven't added any job descriptions yet. Create your first job to get started."}
              </p>
              {!(
                searchQuery ||
                companyFilter ||
                locationFilter ||
                statusFilter ||
                priorityFilter ||
                applicationStatusFilter
              ) && (
                <Button onClick={handleCreateJob}>
                  <Plus className="mr-2 h-4 w-4" />
                  Add Your First Job
                </Button>
              )}
            </div>
          </CardContent>
        </Card>
      ) : (
        <div className="grid gap-4">
          {filteredJobs.map(job => (
            <JobCard
              key={job.id}
              job={job}
              onView={() => handleViewJob(job)}
              onEdit={() => handleEditJob(job)}
              onDelete={() => handleDeleteJob(job)}
              onDuplicate={() => handleDuplicateJob(job)}
              onArchive={() => handleToggleArchive(job)}
              formatSalary={formatSalary}
              formatDate={formatDate}
            />
          ))}
        </div>
      )}
    </div>
  );
}

// JobCard Component
interface JobCardProps {
  job: JobDescription;
  onView: () => void;
  onEdit: () => void;
  onDelete: () => void;
  onDuplicate: () => void;
  onArchive: () => void;
  formatSalary: (min?: number, max?: number, currency?: string) => string;
  formatDate: (dateString?: string) => string;
}

function JobCard({
  job,
  onView,
  onEdit,
  onDelete,
  onDuplicate,
  onArchive,
  formatSalary,
  formatDate,
}: JobCardProps) {
  return (
    <Card
      className={`cursor-pointer transition-all hover:shadow-md ${job.is_archived ? 'opacity-60' : ''}`}
    >
      <CardContent className="p-6">
        <div className="flex items-start justify-between">
          <div className="min-w-0 flex-1" onClick={onView}>
            <div className="mb-2 flex items-center space-x-2">
              <h3 className="truncate text-lg font-semibold">{job.title}</h3>
              {job.is_archived && (
                <Badge variant="secondary" className="text-xs">
                  Archived
                </Badge>
              )}
            </div>
            <div className="mb-3 flex items-center space-x-4 text-sm text-muted-foreground">
              <div className="flex items-center">
                <Building2 className="mr-1 h-4 w-4" />
                {job.company}
              </div>
              {job.location && (
                <div className="flex items-center">
                  <MapPin className="mr-1 h-4 w-4" />
                  {job.location}
                </div>
              )}
              <div className="flex items-center">
                <DollarSign className="mr-1 h-4 w-4" />
                {formatSalary(
                  job.salary_range_min,
                  job.salary_range_max,
                  job.salary_currency
                )}
              </div>
            </div>
            <div className="mb-3 flex items-center space-x-2">
              <Badge className={JOB_STATUS_COLORS[job.status]}>
                {job.status}
              </Badge>
              <Badge className={PRIORITY_COLORS[job.priority]}>
                {job.priority}
              </Badge>
              <Badge
                className={APPLICATION_STATUS_COLORS[job.application_status]}
              >
                {job.application_status.replace(/([A-Z])/g, ' $1').trim()}
              </Badge>
              <Badge variant="outline">{job.remote_options}</Badge>
            </div>
            <div className="flex items-center space-x-4 text-xs text-muted-foreground">
              <span>Updated {formatDate(job.updated_at)}</span>
              {job.posted_date && (
                <span>Posted {formatDate(job.posted_date)}</span>
              )}
              {job.application_deadline && (
                <span>Deadline {formatDate(job.application_deadline)}</span>
              )}
            </div>
          </div>
          <div className="ml-4 flex items-center space-x-1">
            <Button variant="ghost" size="sm" onClick={onView}>
              <Eye className="h-4 w-4" />
            </Button>
            <Button variant="ghost" size="sm" onClick={onEdit}>
              <Edit className="h-4 w-4" />
            </Button>
            <Button variant="ghost" size="sm" onClick={onDuplicate}>
              <Copy className="h-4 w-4" />
            </Button>
            {job.job_url && (
              <Button
                variant="ghost"
                size="sm"
                onClick={() => window.open(job.job_url, '_blank')}
              >
                <ExternalLink className="h-4 w-4" />
              </Button>
            )}
            <Button variant="ghost" size="sm" onClick={onArchive}>
              {job.is_archived ? (
                <ArchiveRestore className="h-4 w-4" />
              ) : (
                <Archive className="h-4 w-4" />
              )}
            </Button>
            <Button variant="ghost" size="sm" onClick={onDelete}>
              <Trash2 className="h-4 w-4" />
            </Button>
          </div>
        </div>
      </CardContent>
    </Card>
  );
}

// Placeholder components that will be implemented in separate tasks
function JobForm({
  job: _job,
  isEdit,
  onSave,
  onCancel,
}: {
  job: JobDescription | null;
  isEdit: boolean;
  onSave: () => void;
  onCancel: () => void;
}) {
  return (
    <Card>
      <CardHeader>
        <CardTitle>{isEdit ? 'Edit Job' : 'Create Job'}</CardTitle>
        <CardDescription>
          {isEdit
            ? 'Update job description details'
            : 'Add a new job description'}
        </CardDescription>
      </CardHeader>
      <CardContent>
        <div className="space-y-4">
          <p className="text-center text-muted-foreground">
            Job form implementation coming soon...
          </p>
          <div className="flex justify-end space-x-2">
            <Button variant="outline" onClick={onCancel}>
              Cancel
            </Button>
            <Button onClick={onSave}>
              {isEdit ? 'Update Job' : 'Create Job'}
            </Button>
          </div>
        </div>
      </CardContent>
    </Card>
  );
}

function JobDetails({
  job,
  onEdit,
  onDelete,
  onDuplicate,
  onArchive,
  onBack,
}: {
  job: JobDescription;
  onEdit: () => void;
  onDelete: () => void;
  onDuplicate: () => void;
  onArchive: () => void;
  onBack: () => void;
}) {
  return (
    <Card>
      <CardHeader>
        <div className="flex items-center justify-between">
          <div>
            <CardTitle>{job.title}</CardTitle>
            <CardDescription>{job.company}</CardDescription>
          </div>
          <div className="flex items-center space-x-2">
            <Button variant="outline" onClick={onBack}>
              Back
            </Button>
            <Button variant="outline" onClick={onEdit}>
              <Edit className="mr-2 h-4 w-4" />
              Edit
            </Button>
          </div>
        </div>
      </CardHeader>
      <CardContent>
        <div className="space-y-4">
          <p className="text-center text-muted-foreground">
            Job details view implementation coming soon...
          </p>
          <div className="flex justify-end space-x-2">
            <Button variant="outline" onClick={onDuplicate}>
              <Copy className="mr-2 h-4 w-4" />
              Duplicate
            </Button>
            <Button variant="outline" onClick={onArchive}>
              {job.is_archived ? (
                <ArchiveRestore className="mr-2 h-4 w-4" />
              ) : (
                <Archive className="mr-2 h-4 w-4" />
              )}
              {job.is_archived ? 'Restore' : 'Archive'}
            </Button>
            <Button variant="destructive" onClick={onDelete}>
              <Trash2 className="mr-2 h-4 w-4" />
              Delete
            </Button>
          </div>
        </div>
      </CardContent>
    </Card>
  );
}

