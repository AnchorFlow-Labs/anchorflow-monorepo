import express from "express";
import cors from "cors";
import type { ApiResponse, AnchorFlow } from "@anchorflow/types";

const app = express();
app.use(cors());
app.use(express.json());

app.get("/health", (_req, res) => {
  res.json({ status: "ok" });
});

app.get("/api/flows", (_req, res) => {
  const response: ApiResponse<AnchorFlow[]> = { data: [] };
  res.json(response);
});

const PORT = process.env.PORT ?? 3001;
app.listen(PORT, () => console.log(`API running on http://localhost:${PORT}`));
