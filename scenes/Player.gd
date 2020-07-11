extends Area2D

# The player emit this signal when it collides with an enemy
signal hit

# How fast the player will move (pixels/sec)
export (int) var speed
# Size of the game window
var screensize
# Controls when the game started
var started

func _ready():
	""" Called every time the node is added to the scene.
		Initialization here. """
	screensize = get_viewport_rect().size
	started = false
	hide()

func _process(delta):
	if started:
		# The player's movement vector
		var velocity = Vector2()
		# Check input
		if Input.is_action_pressed("ui_right"):
			velocity.x += 1
		if Input.is_action_pressed("ui_left"):
			velocity.x -= 1
		if Input.is_action_pressed("ui_down"):
			velocity.y += 1
		if Input.is_action_pressed("ui_up"):
			velocity.y -= 1
		if velocity.length() > 0:
			velocity = velocity.normalized() * speed
			$AnimatedSprite.play()
		else:
			$AnimatedSprite.stop()
		# Change position
		position += velocity * delta
		position.x = clamp(position.x, 0, screensize.x)
		position.y = clamp(position.y, 0, screensize.y)
		# Fix animation
		if velocity.x != 0:
			$AnimatedSprite.animation = "right"
			$AnimatedSprite.flip_v = false
			$AnimatedSprite.flip_h = velocity.x < 0
		elif velocity.y != 0:
			$AnimatedSprite.animation = "up"
			$AnimatedSprite.flip_v = velocity.y > 0

func _on_Player_body_entered(body):
	# Player disappears after being hit
	hide()
	emit_signal("hit")
	started = false
	$CollisionShape2D.disabled = true

func start(pos):
	position = pos
	show()
	$CollisionShape2D.disabled = false