const express = require('express');
const database = require('../utils/database');
const ollamaService = require('../services/ollamaService');

const router = express.Router();

// GET /api/scans - Get all scans
router.get('/', async (req, res) => {
  try {
    const scans = await database.query(
      `SELECT s.*, r.original_name as resume_name, jd.title as job_title, jd.company
       FROM scans s
       JOIN resumes r ON s.resume_id = r.id
       JOIN job_descriptions jd ON s.job_description_id = jd.id
       ORDER BY s.scan_date DESC`
    );
    
    res.json(scans);
  } catch (error) {
    console.error('Error fetching scans:', error);
    res.status(500).json({ error: 'Failed to fetch scans' });
  }
});

// GET /api/scans/:id - Get specific scan with full analysis
router.get('/:id', async (req, res) => {
  try {
    const scan = await database.get(
      `SELECT s.*, r.original_name as resume_name, r.extracted_text as resume_text,
              jd.title as job_title, jd.company, jd.description as job_description
       FROM scans s
       JOIN resumes r ON s.resume_id = r.id
       JOIN job_descriptions jd ON s.job_description_id = jd.id
       WHERE s.id = ?`,
      [req.params.id]
    );
    
    if (!scan) {
      return res.status(404).json({ error: 'Scan not found' });
    }
    
    // Get associated skills
    const skills = await database.query(
      'SELECT * FROM skills WHERE scan_id = ? ORDER BY priority_level DESC, skill_name',
      [req.params.id]
    );
    
    // Parse analysis JSON
    let analysis = {};
    try {
      analysis = JSON.parse(scan.analysis_json);
    } catch (error) {
      console.error('Error parsing analysis JSON:', error);
    }
    
    res.json({
      ...scan,
      analysis,
      skills
    });
  } catch (error) {
    console.error('Error fetching scan:', error);
    res.status(500).json({ error: 'Failed to fetch scan' });
  }
});

// POST /api/scans - Create new scan
router.post('/', async (req, res) => {
  try {
    const { resumeId, jobDescription, model, jobTitle, company } = req.body;
    
    if (!resumeId || !jobDescription || !model) {
      return res.status(400).json({ 
        error: 'Resume ID, job description, and model are required' 
      });
    }
    
    // Get resume
    const resume = await database.get(
      'SELECT * FROM resumes WHERE id = ?',
      [resumeId]
    );
    
    if (!resume) {
      return res.status(404).json({ error: 'Resume not found' });
    }
    
    // Check if Ollama is available
    const isOllamaAvailable = await ollamaService.checkConnection();
    if (!isOllamaAvailable) {
      return res.status(503).json({ 
        error: 'Ollama service is not available. Please ensure Ollama is running.' 
      });
    }
    
    // Save job description
    const jobDescResult = await database.run(
      `INSERT INTO job_descriptions (title, description, company)
       VALUES (?, ?, ?)`,
      [jobTitle || 'Untitled Position', jobDescription, company || 'Unknown Company']
    );
    
    // Perform analysis
    const analysis = await ollamaService.analyzeResume(
      resume.extracted_text,
      jobDescription,
      model
    );
    
    // Save scan
    const scanResult = await database.run(
      `INSERT INTO scans (resume_id, job_description_id, model_used, overall_score, analysis_json)
       VALUES (?, ?, ?, ?, ?)`,
      [resumeId, jobDescResult.lastID, model, analysis.overall_score || 0, JSON.stringify(analysis)]
    );
    
    // Save skills if they exist in the analysis
    if (analysis.found_skills) {
      for (const skill of analysis.found_skills) {
        await database.run(
          `INSERT INTO skills (scan_id, skill_name, skill_type, found_in_resume, priority_level)
           VALUES (?, ?, ?, ?, ?)`,
          [scanResult.lastID, skill.skill, 'found', true, skill.relevance || 'medium']
        );
      }
    }
    
    if (analysis.missing_skills) {
      for (const skill of analysis.missing_skills) {
        await database.run(
          `INSERT INTO skills (scan_id, skill_name, skill_type, found_in_resume, priority_level, recommendation)
           VALUES (?, ?, ?, ?, ?, ?)`,
          [scanResult.lastID, skill.skill, 'missing', false, skill.priority || 'medium', skill.impact]
        );
      }
    }
    
    // Update job description reuse count
    await database.run(
      'UPDATE job_descriptions SET reuse_count = reuse_count + 1 WHERE id = ?',
      [jobDescResult.lastID]
    );
    
    res.json({
      id: scanResult.lastID,
      resumeId,
      jobDescriptionId: jobDescResult.lastID,
      model,
      overallScore: analysis.overall_score || 0,
      analysis
    });
    
  } catch (error) {
    console.error('Error creating scan:', error);
    res.status(500).json({ error: error.message });
  }
});

// DELETE /api/scans/:id - Delete scan
router.delete('/:id', async (req, res) => {
  try {
    const scan = await database.get(
      'SELECT * FROM scans WHERE id = ?',
      [req.params.id]
    );
    
    if (!scan) {
      return res.status(404).json({ error: 'Scan not found' });
    }
    
    await database.run('DELETE FROM scans WHERE id = ?', [req.params.id]);
    
    res.json({ message: 'Scan deleted successfully' });
  } catch (error) {
    console.error('Error deleting scan:', error);
    res.status(500).json({ error: 'Failed to delete scan' });
  }
});

module.exports = router;