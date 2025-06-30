import React from 'react'
import { createContext, useContext, useState, useCallback } from 'react'
import { X, CheckCircle, AlertCircle, AlertTriangle, Info } from 'lucide-react'
import { Button } from './ui/button'

export type NotificationType = 'success' | 'error' | 'warning' | 'info'

export interface Notification {
  id: string
  type: NotificationType
  title: string
  message?: string
  duration?: number
  persistent?: boolean
  action?: {
    label: string
    onClick: () => void
  }
}

interface NotificationContextType {
  notifications: Notification[]
  addNotification: (notification: Omit<Notification, 'id'>) => void
  removeNotification: (id: string) => void
  clearAll: () => void
  success: (title: string, message?: string, options?: Partial<Notification>) => void
  error: (title: string, message?: string, options?: Partial<Notification>) => void
  warning: (title: string, message?: string, options?: Partial<Notification>) => void
  info: (title: string, message?: string, options?: Partial<Notification>) => void
}

const NotificationContext = createContext<NotificationContextType | null>(null)

export function useNotifications() {
  const context = useContext(NotificationContext)
  if (!context) {
    throw new Error('useNotifications must be used within a NotificationProvider')
  }
  return context
}

export function NotificationProvider({ children }: { children: React.ReactNode }) {
  const [notifications, setNotifications] = useState<Notification[]>([])

  const addNotification = useCallback((notification: Omit<Notification, 'id'>) => {
    const id = Math.random().toString(36).substr(2, 9)
    const newNotification: Notification = {
      id,
      duration: 5000, // Default 5 seconds
      ...notification
    }

    setNotifications(prev => [...prev, newNotification])

    // Auto-remove notification after duration (unless persistent)
    if (!newNotification.persistent && newNotification.duration) {
      setTimeout(() => {
        removeNotification(id)
      }, newNotification.duration)
    }
  }, [])

  const removeNotification = useCallback((id: string) => {
    setNotifications(prev => prev.filter(n => n.id !== id))
  }, [])

  const clearAll = useCallback(() => {
    setNotifications([])
  }, [])

  const success = useCallback((title: string, message?: string, options?: Partial<Notification>) => {
    addNotification({ type: 'success', title, message, ...options })
  }, [addNotification])

  const error = useCallback((title: string, message?: string, options?: Partial<Notification>) => {
    addNotification({ 
      type: 'error', 
      title, 
      message, 
      duration: 8000, // Longer duration for errors
      ...options 
    })
  }, [addNotification])

  const warning = useCallback((title: string, message?: string, options?: Partial<Notification>) => {
    addNotification({ type: 'warning', title, message, ...options })
  }, [addNotification])

  const info = useCallback((title: string, message?: string, options?: Partial<Notification>) => {
    addNotification({ type: 'info', title, message, ...options })
  }, [addNotification])

  const value = {
    notifications,
    addNotification,
    removeNotification,
    clearAll,
    success,
    error,
    warning,
    info
  }

  return (
    <NotificationContext.Provider value={value}>
      {children}
      <NotificationContainer />
    </NotificationContext.Provider>
  )
}

function NotificationContainer() {
  const { notifications } = useNotifications()

  if (notifications.length === 0) return null

  return (
    <div className="fixed top-4 right-4 z-50 space-y-2 max-w-sm">
      {notifications.map(notification => (
        <NotificationItem key={notification.id} notification={notification} />
      ))}
    </div>
  )
}

function NotificationItem({ notification }: { notification: Notification }) {
  const { removeNotification } = useNotifications()

  const getIcon = () => {
    switch (notification.type) {
      case 'success':
        return <CheckCircle className="h-5 w-5 text-green-600" />
      case 'error':
        return <AlertCircle className="h-5 w-5 text-red-600" />
      case 'warning':
        return <AlertTriangle className="h-5 w-5 text-yellow-600" />
      case 'info':
        return <Info className="h-5 w-5 text-blue-600" />
    }
  }

  const getBorderColor = () => {
    switch (notification.type) {
      case 'success':
        return 'border-l-green-500'
      case 'error':
        return 'border-l-red-500'
      case 'warning':
        return 'border-l-yellow-500'
      case 'info':
        return 'border-l-blue-500'
    }
  }

  const getBgColor = () => {
    switch (notification.type) {
      case 'success':
        return 'bg-green-50'
      case 'error':
        return 'bg-red-50'
      case 'warning':
        return 'bg-yellow-50'
      case 'info':
        return 'bg-blue-50'
    }
  }

  return (
    <div className={`
      bg-white rounded-lg shadow-lg border-l-4 ${getBorderColor()} ${getBgColor()}
      p-4 transition-all duration-300 ease-in-out
      transform animate-in slide-in-from-right-full
    `}>
      <div className="flex items-start">
        <div className="flex-shrink-0">
          {getIcon()}
        </div>
        
        <div className="ml-3 flex-1">
          <h4 className="text-sm font-medium text-gray-900">
            {notification.title}
          </h4>
          
          {notification.message && (
            <p className="mt-1 text-sm text-gray-600">
              {notification.message}
            </p>
          )}
          
          {notification.action && (
            <div className="mt-3">
              <Button
                size="sm"
                variant="outline"
                onClick={notification.action.onClick}
                className="text-xs"
              >
                {notification.action.label}
              </Button>
            </div>
          )}
        </div>
        
        <div className="ml-4 flex-shrink-0">
          <Button
            variant="ghost"
            size="sm"
            onClick={() => removeNotification(notification.id)}
            className="h-6 w-6 p-0"
          >
            <X className="h-4 w-4" />
          </Button>
        </div>
      </div>
    </div>
  )
}

// Utility hook for handling API errors with notifications
export function useApiErrorHandler() {
  const { error } = useNotifications()

  const handleError = useCallback((err: any, context?: string) => {
    let title = 'An error occurred'
    let message = 'Please try again later'

    if (err?.message) {
      message = err.message
    } else if (typeof err === 'string') {
      message = err
    }

    if (context) {
      title = `Error ${context}`
    }

    // Special handling for common error types
    if (message.toLowerCase().includes('network')) {
      title = 'Network Error'
      message = 'Please check your internet connection and try again'
    } else if (message.toLowerCase().includes('timeout')) {
      title = 'Request Timeout'
      message = 'The request took too long. Please try again'
    } else if (message.toLowerCase().includes('ollama')) {
      title = 'AI Service Unavailable'
      message = 'The AI service is not available. Please ensure Ollama is running'
    }

    error(title, message)
  }, [error])

  return { handleError }
}