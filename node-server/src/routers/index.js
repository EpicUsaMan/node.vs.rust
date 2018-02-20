const router = require("express").Router();

module.exports = news => {
  router.get("/", (req, res) => {
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
