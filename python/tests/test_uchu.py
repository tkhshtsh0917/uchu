from uchu import JapaneseNERProcessor, Token


def test_uchu() -> None:
    # Arrange
    text: str = "私 は 東 京 に 住 ん で い ま す 。"

    expected: list[Token] = [
        Token("私"),
        Token("は"),
        Token("東"),
        Token("京"),
        Token("に"),
        Token("住"),
        Token("ん"),
        Token("で"),
        Token("い"),
        Token("ま"),
        Token("す"),
        Token("。"),
    ]

    ner = JapaneseNERProcessor()

    # Act
    actual: list[Token] = ner.recognize(text)

    # Assert
    assert actual == expected, f"Expected {expected}, but got {actual}"
