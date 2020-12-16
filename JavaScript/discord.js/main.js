// Get Process ID
const process = require('process');
console.info(`Application PID: ${process.pid}`);

// Discord and the client
const Discord   = require('discord.js');
const client    = new Discord.Client();

// Simple ready handler
client.on('ready', () => {
    console.info(`Connected as '${client.user.tag}' on ${client.guilds.cache.size} guilds`);
});

// Message handler
client.on('message', (m) => {
    console.log(`'${m.author.username}'@'${m.guild.name}': '${m.content}'`);
})

// Connect the bot
client.login(process.env.DISCORD_TOKEN);