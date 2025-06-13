const fs = require('fs');
const path = require('path');
const pdfParse = require('pdf-parse');
const mammoth = require('mammoth');

class FileProcessor {
  static async processFile(filePath) {
    const ext = path.extname(filePath).toLowerCase();
    
    try {
      switch (ext) {
        case '.pdf':
          return await this.processPDF(filePath);
        case '.doc':
        case '.docx':
          return await this.processWord(filePath);
        default:
          throw new Error(`Unsupported file format: ${ext}`);
      }
    } catch (error) {
      console.error('File processing error:', error);
      throw new Error(`Failed to process file: ${error.message}`);
    }
  }

  static async processPDF(filePath) {
    const dataBuffer = fs.readFileSync(filePath);
    const data = await pdfParse(dataBuffer);
    return {
      text: data.text,
      pages: data.numpages,
      info: data.info
    };
  }

  static async processWord(filePath) {
    const result = await mammoth.extractRawText({ path: filePath });
    return {
      text: result.value,
      messages: result.messages
    };
  }

  static validateFileType(filename) {
    const allowedExtensions = ['.pdf', '.doc', '.docx'];
    const ext = path.extname(filename).toLowerCase();
    return allowedExtensions.includes(ext);
  }

  static getFileSize(filePath) {
    const stats = fs.statSync(filePath);
    return stats.size;
  }
}

module.exports = FileProcessor;