const express = require('express');
const multer = require('multer');
const path = require('path');
const fs = require('fs');
const { v4: uuidv4 } = require('uuid');
const database = require('../utils/database');
const FileProcessor = require('../utils/fileProcessor');

const router = express.Router();

// Configure multer for file uploads
const storage = multer.diskStorage({
  destination: (req, file, cb) => {
    const uploadDir = path.join(__dirname, '../../uploads');
    if (!fs.existsSync(uploadDir)) {
      fs.mkdirSync(uploadDir, { recursive: true });
    }
    cb(null, uploadDir);
  },
  filename: (req, file, cb) => {
    const uniqueName = `${uuidv4()}-${file.originalname}`;
    cb(null, uniqueName);
  }
});

const upload = multer({
  storage: storage,
  limits: {
    fileSize: 10 * 1024 * 1024 // 10MB limit
  },
  fileFilter: (req, file, cb) => {
    if (FileProcessor.validateFileType(file.originalname)) {
      cb(null, true);
    } else {
      cb(new Error('Unsupported file type. Only PDF and Word documents are allowed.'));
    }
  }
});

// GET /api/resumes - Get all resumes
router.get('/', async (req, res) => {
  try {
    const resumes = await database.query(
      `SELECT r.*, s.id as last_scan_id, s.overall_score as last_score, s.scan_date as last_scan_date,
              jd.title as last_job_title, jd.company as last_company
       FROM resumes r
       LEFT JOIN scans s ON r.id = s.resume_id AND s.scan_date = (
         SELECT MAX(scan_date) FROM scans WHERE resume_id = r.id
       )
       LEFT JOIN job_descriptions jd ON s.job_description_id = jd.id
       ORDER BY r.upload_date DESC`
    );
    
    res.json(resumes);
  } catch (error) {
    console.error('Error fetching resumes:', error);
    res.status(500).json({ error: 'Failed to fetch resumes' });
  }
});

// POST /api/resumes - Upload new resume
router.post('/', upload.single('resume'), async (req, res) => {
  try {
    if (!req.file) {
      return res.status(400).json({ error: 'No file uploaded' });
    }

    const filePath = req.file.path;
    const fileSize = FileProcessor.getFileSize(filePath);
    
    // Process the file to extract text
    const processedFile = await FileProcessor.processFile(filePath);
    
    // Save to database
    const result = await database.run(
      `INSERT INTO resumes (filename, original_name, file_path, file_size, extracted_text)
       VALUES (?, ?, ?, ?, ?)`,
      [req.file.filename, req.file.originalname, filePath, fileSize, processedFile.text]
    );

    const resume = await database.get(
      'SELECT * FROM resumes WHERE id = ?',
      [result.lastID]
    );

    res.json({
      id: result.lastID,
      ...resume,
      preview: processedFile.text.substring(0, 500) + '...'
    });
  } catch (error) {
    console.error('Error uploading resume:', error);
    
    // Clean up uploaded file if processing failed
    if (req.file && req.file.path) {
      try {
        fs.unlinkSync(req.file.path);
      } catch (cleanupError) {
        console.error('Error cleaning up file:', cleanupError);
      }
    }
    
    res.status(500).json({ error: error.message });
  }
});

// GET /api/resumes/:id - Get specific resume
router.get('/:id', async (req, res) => {
  try {
    const resume = await database.get(
      'SELECT * FROM resumes WHERE id = ?',
      [req.params.id]
    );
    
    if (!resume) {
      return res.status(404).json({ error: 'Resume not found' });
    }
    
    res.json(resume);
  } catch (error) {
    console.error('Error fetching resume:', error);
    res.status(500).json({ error: 'Failed to fetch resume' });
  }
});

// DELETE /api/resumes/:id - Delete resume
router.delete('/:id', async (req, res) => {
  try {
    const resume = await database.get(
      'SELECT * FROM resumes WHERE id = ?',
      [req.params.id]
    );
    
    if (!resume) {
      return res.status(404).json({ error: 'Resume not found' });
    }
    
    // Delete file from filesystem
    if (fs.existsSync(resume.file_path)) {
      fs.unlinkSync(resume.file_path);
    }
    
    // Delete from database (cascades to scans and skills)
    await database.run('DELETE FROM resumes WHERE id = ?', [req.params.id]);
    
    res.json({ message: 'Resume deleted successfully' });
  } catch (error) {
    console.error('Error deleting resume:', error);
    res.status(500).json({ error: 'Failed to delete resume' });
  }
});

module.exports = router;