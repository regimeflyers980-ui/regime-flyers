# Build Stage
FROM node:20-alpine AS builder
WORKDIR /app

# Copy ONLY package files first to leverage Docker layer caching
COPY package*.json ./

# Permanent Fix: Mount the NPM cache so it survives across builds
RUN --mount=type=cache,target=/root/.npm npm ci

COPY . .
RUN npm run build

# Run Stage
FROM node:20-alpine AS runner
WORKDIR /app
ENV NODE_ENV=production

# Copy the standalone output
COPY --from=builder /app/public ./public
COPY --from=builder /app/.next/standalone ./
COPY --from=builder /app/.next/static ./.next/static

EXPOSE 3000
CMD ["node", "server.js"]
