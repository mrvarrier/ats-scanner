import axios from 'axios';
import { Resume, Scan, AnalysisResult, ModelResponse } from '../types';

const API_BASE = 'http://localhost:3001/api';

const api = axios.create({
  baseURL: API_BASE,
});

export const resumeAPI = {
  getAll: (): Promise<Resume[]> => 
    api.get('/resumes').then(res => res.data),
    
  upload: (file: File): Promise<Resume> => {
    const formData = new FormData();
    formData.append('resume', file);
    return api.post('/resumes', formData, {
      headers: { 'Content-Type': 'multipart/form-data' }
    }).then(res => res.data);
  },
  
  getById: (id: number): Promise<Resume> =>
    api.get(`/resumes/${id}`).then(res => res.data),
    
  delete: (id: number): Promise<void> =>
    api.delete(`/resumes/${id}`).then(res => res.data),
};

export const scanAPI = {
  getAll: (): Promise<Scan[]> =>
    api.get('/scans').then(res => res.data),
    
  getById: (id: number): Promise<Scan> =>
    api.get(`/scans/${id}`).then(res => res.data),
    
  create: (data: {
    resumeId: number;
    jobDescription: string;
    model: string;
    jobTitle?: string;
    company?: string;
  }): Promise<Scan> =>
    api.post('/scans', data).then(res => res.data),
    
  delete: (id: number): Promise<void> =>
    api.delete(`/scans/${id}`).then(res => res.data),
};

export const analyzeAPI = {
  analyze: (data: {
    resumeText: string;
    jobDescription: string;
    model?: string;
  }): Promise<AnalysisResult> =>
    api.post('/analyze', data).then(res => res.data),
    
  getModels: (): Promise<ModelResponse> =>
    api.get('/analyze/models').then(res => res.data),
};

export const healthAPI = {
  check: (): Promise<{ status: string; timestamp: string; version: string }> =>
    api.get('/health').then(res => res.data),
};