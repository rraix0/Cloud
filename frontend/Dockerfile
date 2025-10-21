FROM oven/bun:1-alpine as build-stage
WORKDIR /app
COPY package*.json ./
RUN bun install
COPY . .
RUN bun run build
FROM nginx:stable-alpine as production-stage
COPY --from=build-stage /app/dist /usr/share/nginx/html
COPY nginx.conf /etc/nginx/conf.d/default.conf
EXPOSE 80

ENV VITE_BASE_URL=/
ENV VITE_HOST="http://localhost:8000/api"
CMD ["nginx", "-g", "daemon off;"]
