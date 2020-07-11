extends CanvasLayer

# This signal tells that the start button has been pressed
signal start_game

func _ready():
	""" Called every time the node is added to the scene.
		Initialization here. """
	$ScoreLabel.hide()

func show_message(text):
	$MessageLabel.text = text
	$MessageLabel.show()
	$MessageTimer.start()

func show_game_over():
	show_message("Game Over")
	yield($MessageTimer, "timeout")
	$StartButton.show()
	$MessageLabel.text = "Dodge the Creeps!"
	$MessageLabel.show()

func update_score(score):
	$ScoreLabel.text = str(score)

func _on_StartButton_pressed():
	$StartButton.hide()
	$ScoreLabel.show()
	emit_signal("start_game")

func _on_MessageTimer_timeout():
	$MessageLabel.hide()