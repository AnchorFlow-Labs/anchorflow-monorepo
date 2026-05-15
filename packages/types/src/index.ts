// Shared types between frontend and backend

export interface AnchorFlow {
  id: string;
  name: string;
  contractAddress: string;
  createdAt: string;
}

export interface ApiResponse<T> {
  data: T;
  error?: string;
}
