const router = require("express").Router();

module.exports = news => {
  router.get("/news", (req, res) => {
    if (!req.query.id) {
      news.findAll().then(result => res.json(result));
    } else {
      news
        .findAll({
          where: {
            id: req.query.id
          }
        })
        .then(result => res.json(result));
    }
  });

  router.post("/news", (req, res) => {
    news.update(
      {
        title: req.body.title,
        text: req.body.text
      },
      {
        where: {
          id: req.body.id
        }
      }
    );

    res.json();
  });

  router.put("/news", (req, res) => {
    news.create({
      title: req.body.title,
      text: req.body.text
    });

    res.json();
  });

  router.delete("/news", (req, res) => {
    news.destroy({
      where: req.body.id
    });

    res.json();
  });

  return router;
};
