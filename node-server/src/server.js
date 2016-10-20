var express = require('express'),
    Sequelize = require('sequelize'),
    bodyParser = require('body-parser'),
    app = express(),
    sequelize = new Sequelize('postgres://localhost/news');

app.use(bodyParser.json({ type: 'application/*+json' }))

var news = sequelize.define('news', {
  id: {
    type: Sequelize.INTEGER,
    primaryKey: true,
    autoIncrement: true
  },
  time: {
    type: Sequelize.BIGINT,
    defaultValue: Sequelize.NOW
  },
  title: {
    type: Sequelize.STRING,
    allowNull: false
  },
  text: {
    type: Sequelize.TEXT,
    allowNull: false
  }
}, {
  freezeTableName: true
});

app.get('/', function (req, res) {
  if (req.body.id == undefined) {
    news.findAll().then(result => function() {
      app.json(result);
    });
  } else {
    news.findAll({
      where: {
        id: req.body.id
      }
    }).then(result => function() {
      app.json(result);
    });
  }
});

app.post('/', function (req, res) {
  news.findAll({
    where: {
      id: req.body.id
    }
  }).set({
    title: req.body.title,
    text: req.body.text
  }).save();
});

app.put('/', function (req, res) {
  news.create({
    title: req.body.title,
    text: req.body.text
  }).save();
});

app.delete('/', function (req, res) {
  news.findAll({
    where: req.body.id
  }).destroy().save();
});

app.listen(3000);
