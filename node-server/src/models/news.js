module.exports = (sequelize, types) => {
  return sequelize.define("news", {
    id: {
      type: types.INTEGER,
      primaryKey: true,
      autoIncrement: true
    },
    time: {
      type: types.BIGINT,
      defaultValue: types.NOW
    },
    title: {
      type: types.STRING,
      allowNull: false
    },
    text: {
      type: types.TEXT,
      allowNull: false
    }
  });
};
