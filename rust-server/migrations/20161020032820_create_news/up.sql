CREATE TABLE IF NOT EXISTS "news" (
    "id"  SERIAL , 
    "title" VARCHAR(255) NOT NULL, 
    "text" TEXT NOT NULL, 
    "createdAt" TIMESTAMP WITH TIME ZONE NOT NULL, 
    "updatedAt" TIMESTAMP WITH TIME ZONE NOT NULL, 
    PRIMARY KEY ("id")
);