FROM node:lts

WORKDIR /app

COPY package*.json ./
RUN npm install

COPY . .

ENV HOST=0.0.0.0
ENV PORT=3000

EXPOSE 3000

CMD ["npx", "nuxi", "dev", "--host", "0.0.0.0"]
