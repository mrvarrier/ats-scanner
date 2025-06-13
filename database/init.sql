-- Personal ATS Scanner Database Schema

-- Resumes table
CREATE TABLE IF NOT EXISTS resumes (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    filename TEXT NOT NULL,
    original_name TEXT NOT NULL,
    file_path TEXT NOT NULL,
    upload_date DATETIME DEFAULT CURRENT_TIMESTAMP,
    file_size INTEGER NOT NULL,
    extracted_text TEXT NOT NULL
);

-- Job descriptions table
CREATE TABLE IF NOT EXISTS job_descriptions (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    title TEXT,
    description TEXT NOT NULL,
    company TEXT,
    date_added DATETIME DEFAULT CURRENT_TIMESTAMP,
    reuse_count INTEGER DEFAULT 0
);

-- Scans table
CREATE TABLE IF NOT EXISTS scans (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    resume_id INTEGER NOT NULL,
    job_description_id INTEGER NOT NULL,
    model_used TEXT NOT NULL,
    overall_score INTEGER NOT NULL,
    analysis_json TEXT NOT NULL,
    scan_date DATETIME DEFAULT CURRENT_TIMESTAMP,
    FOREIGN KEY (resume_id) REFERENCES resumes(id) ON DELETE CASCADE,
    FOREIGN KEY (job_description_id) REFERENCES job_descriptions(id) ON DELETE CASCADE
);

-- Skills table
CREATE TABLE IF NOT EXISTS skills (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    scan_id INTEGER NOT NULL,
    skill_name TEXT NOT NULL,
    skill_type TEXT NOT NULL, -- 'found', 'missing', 'recommended'
    found_in_resume BOOLEAN DEFAULT FALSE,
    priority_level TEXT DEFAULT 'medium', -- 'high', 'medium', 'low'
    recommendation TEXT,
    FOREIGN KEY (scan_id) REFERENCES scans(id) ON DELETE CASCADE
);

-- Indexes for better performance
CREATE INDEX IF NOT EXISTS idx_resumes_upload_date ON resumes(upload_date);
CREATE INDEX IF NOT EXISTS idx_scans_resume_id ON scans(resume_id);
CREATE INDEX IF NOT EXISTS idx_scans_scan_date ON scans(scan_date);
CREATE INDEX IF NOT EXISTS idx_skills_scan_id ON skills(scan_id);