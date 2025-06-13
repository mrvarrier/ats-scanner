const express = require('express');
const ollamaService = require('../services/ollamaService');

const router = express.Router();

// POST /api/analyze - Quick analysis endpoint
router.post('/', async (req, res) => {
  try {
    const { resumeText, jobDescription, model } = req.body;
    
    if (!resumeText || !jobDescription) {
      return res.status(400).json({ 
        error: 'Resume text and job description are required' 
      });
    }
    
    // Check if Ollama is available
    const isOllamaAvailable = await ollamaService.checkConnection();
    if (!isOllamaAvailable) {
      return res.status(503).json({ 
        error: 'Ollama service is not available. Please ensure Ollama is running.' 
      });
    }
    
    // Perform analysis
    const analysis = await ollamaService.analyzeResume(
      resumeText,
      jobDescription,
      model || 'mistral:latest'
    );
    
    res.json(analysis);
    
  } catch (error) {
    console.error('Error performing analysis:', error);
    res.status(500).json({ error: error.message });
  }
});

// GET /api/models - Get available Ollama models
router.get('/models', async (req, res) => {
  try {
    const isOllamaAvailable = await ollamaService.checkConnection();
    if (!isOllamaAvailable) {
      return res.status(503).json({ 
        error: 'Ollama service is not available',
        available: false,
        models: []
      });
    }
    
    const models = await ollamaService.getAvailableModels();
    const availableModels = models.filter(model => 
      ollamaService.availableModels.some(available => 
        model.name.includes(available.split(':')[0])
      )
    );
    
    res.json({
      available: true,
      models: availableModels,
      recommended: [
        {
          name: 'mistral:latest',
          description: 'Faster analysis, good for quick scans',
          speed: 'fast'
        },
        {
          name: 'qwen2.5:14b',
          description: 'More detailed analysis, better insights',
          speed: 'slower'
        }
      ]
    });
    
  } catch (error) {
    console.error('Error fetching models:', error);
    res.status(500).json({ 
      error: error.message,
      available: false,
      models: []
    });
  }
});

module.exports = router;