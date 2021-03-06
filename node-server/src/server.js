const cluster = require("cluster");

if (cluster.isMaster) {
  const cpuCount = require("os").cpus().length;

  // Create a worker for each CPU
  for (let i = 0; i < cpuCount; i++) {
    cluster.fork();
  }
} else {
  const app = require("express")(),
    Sequelize = require("sequelize"),
    bodyParser = require("body-parser"),
    sequelize = new Sequelize("postgres://test:test@localhost/news", {
      logging: false,
      operatorsAliases: false /** Fix of @see https://github.com/sequelize/sequelize/issues/8417 */
    }),
    news = require("./models/news")(sequelize, Sequelize);

  app.use(bodyParser.json());
  app.use(require("./routers")(news));

  news
    .sync({
      force: true
    })
    .then(e => {
      app.listen(3000);
    });
}
