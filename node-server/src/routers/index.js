const router = require("express").Router();

module.exports = news => {
  router.get("/", (req, res) => {
    if (!req.body.id) {
      news.findAll().then(result => router.json(result));
    } else {
      news
        .findAll({
          where: {
            id: req.body.id
          }
        })
        .then(result => router.json(result));
    }
  });

  router.post("/", (req, res) => {
    news
      .findAll({
        where: {
          id: req.body.id
        }
      })
      .set({
        title: req.body.title,
        text: req.body.text
      })
      .save();

    res.json();
  });

  router.put("/", (req, res) => {
    news
      .create({
        title: req.body.title,
        text: req.body.text
      })
      .save();

    res.json();
  });

  router.delete("/", (req, res) => {
    news
      .findAll({
        where: req.body.id
      })
      .destroy()
      .save();

    res.json();
  });

  return router;
};
