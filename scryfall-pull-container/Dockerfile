# syntax=docker/dockerfile:1

FROM node:20-alpine

# Set destination for COPY
WORKDIR /app

# Copy container source code
COPY container_src/index.ts ./

# In a container, we just run the TS file using ts-node or similar, 
# but for maximum performance we'd transpile it first.
# For simplicity in a Cloudflare Container, we'll install esbuild to transpile on the fly
# or just install ts-node.

RUN npm install -g ts-node typescript @types/node

EXPOSE 8080

# Run
CMD ["ts-node", "index.ts"]
