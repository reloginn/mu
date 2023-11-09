pub const DEFAULT_TEMPLATE: &str = r#"box.cfg()

local httpd = require('http.server')

box.once('schema', function()
    local sessions = box.schema.create_space('sessions', {
        format = {
            { name = 'session', type = 'string' },
            { name = 'uuid', type = 'string' },
            { name = 'user_agent', type = 'string' }
        }, if_not_exists = true
    })
    sessions:create_index('session', { type = 'hash', parts = {'session'}, if_not_exists = true })
    sessions:create_index('uuid', { type = 'hash', parts = {'uuid'}, if_not_exists = true })
end)

local sessions = box.space.sessions

local server = httpd.new('0.0.0.0', 5432, {
    log_errors = true,
})

server:route({ path = '/', method = 'PUT' }, function(self)
    local body = self:json()
    if not body or not body.session or not body.uuid or not body.user_agent then
        return self:render({ json = { error = 'The request body does not have the required fields or is missing' } })
    end
    local success, err = pcall(function()
        sessions:insert({ body.session, body.uuid, body.user_agent })
    end)
    if success then
        return nil
    else
        return self:render({ json = { error = 'Internal server error: ' .. err } })
    end
end)
server:route({ path = '/', method = 'GET' }, function(self)
    local body = self:json()
    if not body then
        return self:render({ json = { error = "The request body cannot be empty" } })
    end
    if body.session then
        local data = sessions.index.sessions:select(body.session)[1]
        if data then
            return self:render({ json = data })
        else
            return self:render({ json = { error = 'No one was found' } })
        end
    elseif body.uuid then
        local data = sessions.index.uuid:select(body.uuid)[1]
        if data then
            return self:render({ json = data })
        else
            return self:render({ json = { error = 'This user has no sessions' } })
        end
    else
        return self:render({ json = { error = "The request body does not have the required fields" } })
    end
end)
"#;
