const router = require("express").Router();

module.exports = news => {
  router.get("/news/:limit?", (req, res) => {
    if (!req.params.id) {
      news.findAll().then(result => res.json(result));
    } else {
      news
        .findAll({
          limit: req.params.id
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
