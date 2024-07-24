FROM node:alpine

WORKDIR /app

COPY . .

RUN npm install -g @angular/cli

RUN npm install

CMD ["ng", "serve", "--configuration=development" , "--host", "0.0.0.0", "--poll", "2000", "--disable-host-check"]