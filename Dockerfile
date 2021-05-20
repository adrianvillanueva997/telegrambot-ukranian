# Multistage docker image building
# build-env -> dist

FROM node:16.2.0-alpine as base
# Building container
FROM base as builder
WORKDIR /build
COPY package.json .
RUN npm install
COPY . .
RUN npm run build

# Production container
FROM base as dist
WORKDIR /app
COPY package.json .
RUN npm install --production
COPY --from=builder /build/dist ./dist
RUN adduser -D appuser
USER appuser
CMD ["npm", "run", "prod"]
